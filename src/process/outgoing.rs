use tokio_postgres::Client;
use log::{error, info};

pub async fn process_outgoing(data: &[u8], client: &Client) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let request_text = String::from_utf8_lossy(data);
    if request_text.contains("UpsertChat") {
        info!("Starting UpsertChat process...");
        if let Ok(chat) = serde_json::from_str::<crate::parser::library::Chat>(&request_text) {
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
        } else {
            error!("Failed to deserialize chat from JSON: {}", request_text);
            return Err("Couldn't deserialize data.".into());
        }
    } else if request_text.contains("UpsertCustomer") {
        info!("Starting UpsertCustomer process...");
        if let Ok(customer) = serde_json::from_str::<crate::parser::library::Customer>(&request_text) {
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
        } else {
            error!("Failed to deserialize customer from JSON: {}", request_text);
            return Err("Couldn't deserialize data.".into());
        }
    } else if request_text.contains("UpsertMessage") {
        info!("Starting UpsertMessage process...");
        if let Ok(message) = serde_json::from_str::<crate::parser::library::Message>(&request_text) {
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
        } else {
            error!("Failed to deserialize message from JSON: {}", request_text);
            return Err("Couldn't deserialize data.".into());
        }
    } else if request_text.contains("SendRequest") {
        info!("Starting SendRequest process...");
        if let Ok(request) = serde_json::from_str::<crate::parser::library::Request>(&request_text) {
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
        } else {
            error!("Failed to deserialize request from JSON: {}", request_text);
            return Err("Couldn't deserialize data.".into());
        }
    } else {
        error!("Failed to deserialize request from JSON: {}", request_text);
        return Err("Couldn't deserialize data.".into());
    }
}
