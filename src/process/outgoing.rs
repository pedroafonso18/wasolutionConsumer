use tokio_postgres::Client;
use log::{error, info, debug};

pub async fn process_outgoing(data: &[u8], client: &Client) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let request_text = String::from_utf8_lossy(data);
    
    debug!("Received message: {}", request_text);
    info!("Processing message of {} bytes", data.len());
    
    if request_text.contains("upsertChat") {
        info!("Starting UpsertChat process...");
        match serde_json::from_str::<crate::parser::library::Chat>(&request_text) {
            Ok(chat) => {
                info!("Successfully deserialized chat with ID: {}", chat.id);
                match crate::database::insert::upsert_chats(&client, &chat).await {
                    Ok(_) => {
                        info!("Succesfully upserted chat into the db!");
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error on upserting chat into the db: {}",e);
                        return Err("Couldn't upsert data into the db.".into());
                    }
                }
            }
            Err(e) => {
                error!("Failed to deserialize chat from JSON: {}", e);
                error!("Raw message: {}", request_text);
                return Err(format!("Couldn't deserialize chat data: {}", e).into());
            }
        }
    } else if request_text.contains("upsertCustomer") {
        info!("Starting UpsertCustomer process...");
        match serde_json::from_str::<crate::parser::library::Customer>(&request_text) {
            Ok(customer) => {
                info!("Successfully deserialized customer with ID: {}", customer.id);
                match crate::database::insert::upsert_customer(&client, &customer).await {
                    Ok(_) => {
                        info!("Succesfully upserted customer into the db!");
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error on upserting customer into the db: {}",e);
                        return Err("Couldn't upsert customer into the db.".into());
                    }
                }
            }
            Err(e) => {
                error!("Failed to deserialize customer from JSON: {}", e);
                error!("Raw message: {}", request_text);
                return Err(format!("Couldn't deserialize customer data: {}", e).into());
            }
        }
    } else if request_text.contains("upsertMessage") {
        info!("Starting UpsertMessage process...");
        match serde_json::from_str::<crate::parser::library::Message>(&request_text) {
            Ok(message) => {
                info!("Successfully deserialized message with ID: {}", message.id);
                match crate::database::insert::upsert_messages(&client, &message).await {
                    Ok(_) => {
                        info!("Succesfully upserted message into the db!");
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error on upserting message into the db: {}",e);
                        return Err("Couldn't upsert message into the db.".into());
                    }
                }
            }
            Err(e) => {
                error!("Failed to deserialize message from JSON: {}", e);
                error!("Raw message: {}", request_text);
                return Err(format!("Couldn't deserialize message data: {}", e).into());
            }
        }
    } else if request_text.contains("sendRequest") {
        info!("Starting SendRequest process...");
        match serde_json::from_str::<crate::parser::library::Request>(&request_text) {
            Ok(request) => {
                info!("Successfully deserialized request for: {}", request.action);
                match crate::api::requests::make_request(request).await {
                    Ok(_) => {
                        info!("Succesfully processed the request!");
                        return Ok(());
                    }
                    Err(e) => {
                        error!("Error on processing request: {}",e);
                        return Err("Couldn't process request.".into());
                    }
                }
            }
            Err(e) => {
                error!("Failed to deserialize request from JSON: {}", e);
                error!("Raw message: {}", request_text);
                return Err(format!("Couldn't deserialize request data: {}", e).into());
            }
        }
    } else {
        error!("Unknown message type. Message content: {}", request_text);
        error!("Message doesn't contain any of the expected keywords: UpsertChat, UpsertCustomer, UpsertMessage, SendRequest");
        return Err("Couldn't deserialize data - unknown message type.".into());
    }
}
