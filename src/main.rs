mod config;
mod rabbit;
mod api;
mod parser;
mod database;
mod process;

use log::{error, info, warn};
use tokio::time::{sleep, Duration};
use futures::pin_mut;
use futures::StreamExt;
use tokio::select;
use tokio::signal;
use tokio_postgres::Client;
use std::sync::Arc;


#[tokio::main]
async fn main() {
    let env = match config::config::load_dotenv() {
        Ok(env) => env,
        Err(e) => {
            error!("ERROR: Couldn't retrieve .env: {}", e);
            return;
        }
    };

    info!("Starting application - Check Logs below...");

    info!("Starting WaSolConsumer");

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
        info!("Setting up Outgoing Request consumer...");
        match run_consumer(&env.rabbit_url, &db_client, "outgoing_requests").await {
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
    queue_name: &str
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

                        tokio::spawn(async move {
                            if queue_name == "outgoing_requests" {
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
                            } else if queue_name == "incoming_requests" {
                                /*
                                match process::incoming::process_incoming(&data, &db).await {
                                    Ok(_) => {
                                        info!("Successfully processed incoming request");
                                    }
                                    Err(e) => {
                                        error!("Error processing incoming request: {}", e);
                                    }
                                }
                                */
                                info!("Still not implemented...");
                            }
                        });

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