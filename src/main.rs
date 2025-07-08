mod config;
mod rabbit;
mod api;
mod parser;
mod database;
mod process;
mod redis_mod;

use log::{error, info, warn};
use tokio::time::{sleep, Duration};
use futures::pin_mut;
use futures::StreamExt;
use tokio::select;
use tokio::signal;
use tokio_postgres::Client;
use std::sync::Arc;
use env_logger::{Builder, Env};
use redis::aio::MultiplexedConnection;
use tokio::sync::Mutex;
use crate::parser::library::SendMessageResponse;
use crate::redis_mod::redis::insert_message_to_chat;

#[tokio::main]
async fn main() {
    let envi = Env::default().filter_or("RUST_LOG", "debug");

    Builder::from_env(envi)
        .format_timestamp_secs()
        .format_module_path(true)
        .init();

    let env = match config::config::load_dotenv() {
        Ok(env) => env,
        Err(e) => {
            error!("ERROR: Couldn't retrieve .env: {}", e);
            return;
        }
    };

    info!("Starting application - Check Logs below...");

    info!("Starting WaSolConsumer");

    let redis_conn = match crate::redis_mod::redis::connect_redis(&env.redis_url).await {
        Ok(conn) => Arc::new(Mutex::new(conn)),
        Err(e) => {
            error!("ERROR: Couldn't connect to Redis: {}", e);
            return;
        }
    };

    loop {
        let db_client = match database::connect::connect_db(&env.db_url).await {
            Ok(db_client) => db_client,
            Err(e) => {
                error!("ERROR: Couldn't connect to Database, retrying... : {}",e);
                sleep(Duration::from_secs(30)).await;
                continue;
            }
        };
        let db_client = Arc::new(db_client);
        info!("Setting up Outgoing and Incoming Request consumers...");

        let outgoing = run_consumer(&env.rabbit_url, &db_client, "outgoing_requests", None);
        let incoming = run_consumer(&env.rabbit_url, &db_client, "incoming_requests", Some(Arc::clone(&redis_conn)));
        let upsert = run_consumer(&env.rabbit_url, &db_client, "evolution.messages.upsert", Some(Arc::clone(&redis_conn)));
        let send = run_consumer(&env.rabbit_url, &db_client, "evolution.send.message", Some(Arc::clone(&redis_conn)));

        let result = tokio::try_join!(outgoing, incoming, upsert, send);
        match result {
            Ok(_) => {
                info!("Application shutdown requested");
                break;
            }
            Err(e) => {
                error!("Error in consumer loop: {}", e);
                println!("ERROR: Consumer loop failed: {}", e);
                info!("Reconnecting in 5 seconds...");
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

async fn run_consumer(
    rabbit_url: &str,
    db_client: &Arc<Client>,
    queue_name: &str,
    redis_conn: Option<Arc<Mutex<MultiplexedConnection>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (mut consumer, _) = match rabbit::setup_rabbit::create_rabbitmq_consumer(rabbit_url, queue_name).await {
        Some((consumer, connection)) => (consumer, connection),
        None => return Err("Failed to create RabbitMQ consumer".into()),
    };

    info!("Consumer ready, waiting for webhooks...");
    info!("Press Ctrl+C to exit");

    loop {
        let ctrl_c_future = signal::ctrl_c();
        pin_mut!(ctrl_c_future);

        select! {
            delivery_result = consumer.next() => {
                match delivery_result {
                    Some(Ok(delivery)) => {
                        let data = delivery.data.clone();
                        let db = Arc::clone(db_client);
                        let queue_name = queue_name.to_string();
                        if queue_name == "incoming_requests" {
                            let redis_conn = redis_conn.as_ref().unwrap().clone();
                            tokio::spawn(async move {
                                let mut redis_conn = redis_conn.lock().await;
                                match process::incoming::process_incoming(&data, &mut *redis_conn).await {
                                    Ok(_) => {
                                        info!("Successfully processed incoming request");
                                    }
                                    Err(e) => {
                                        error!("Error processing incoming request: {}", e);
                                    }
                                }
                            });
                        } else if queue_name == "outgoing_requests" {
                            tokio::spawn(async move {
                                match process::outgoing::process_outgoing(&data, &db).await {
                                    Ok(_) => {
                                        info!("Successfully processed outgoing request");
                                    }
                                    Err(e) => {
                                        error!("Error in consumer loop: {}", e);
                                        println!("ERROR: Consumer loop failed: {}", e);
                                        info!("Reconnecting in 5 seconds...");
                                        sleep(Duration::from_secs(5)).await;
                                    }
                                }
                            });
                        } else if queue_name == "evolution.messages.upsert" {
                            let redis_conn = redis_conn.as_ref().unwrap().clone();
                            tokio::spawn(async move {
                                let mut redis_conn = redis_conn.lock().await;
                                match process::incoming::process_incoming(&data, &mut *redis_conn).await {
                                    Ok(_) => {
                                        info!("Successfully processed outgoing request");
                                    }
                                    Err(e) => {
                                        error!("Error in consumer loop: {}", e);
                                        println!("ERROR: Consumer loop failed: {}", e);
                                        info!("Reconnecting in 5 seconds...");
                                        sleep(Duration::from_secs(5)).await;
                                    }
                                }
                            });
                        } else if queue_name == "evolution.send.message" {
                            let redis_conn = redis_conn.as_ref().unwrap().clone();
                            tokio::spawn(async move {
                                let mut redis_conn = redis_conn.lock().await;
                                match serde_json::from_slice::<SendMessageResponse>(&data) {
                                    Ok(response) => {
                                        let chat_id = &response.status_string.key.remote_jid;
                                        let remote_jid = chat_id;
                                        let message_json = serde_json::to_string(&response.status_string.message).unwrap_or_default();
                                        if let Err(e) = insert_message_to_chat(&mut *redis_conn, chat_id, &message_json, remote_jid).await {
                                            error!("Failed to insert message to Redis: {}", e);
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to deserialize SendMessageResponse: {}", e);
                                    }
                                }
                            });
                        }

                        if let Err(e) = delivery.ack(lapin::options::BasicAckOptions::default()).await {
                            error!("Failed to acknowledge message: {}", e);
                        }
                    },
                    Some(Err(e)) => {
                        error!("Error receiving message: {}", e);
                        return Err(Box::new(e));
                    },
                    None => {
                        warn!("Consumer channel closed");
                        return Err("Consumer channel closed unexpectedly".into());
                    }
                }
            },

            _ = ctrl_c_future => {
                info!("Received shutdown signal");
                break;
            }
        }
    }

    Ok(())
}