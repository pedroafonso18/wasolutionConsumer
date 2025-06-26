use reqwest;
use crate::parser;
use log::{info, error};

pub async fn make_request(request : parser::library::Request) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    info!("Started making request for : {}", request.action);

    if request.method == "POST" {
        let mut req = client.post(&request.url);

        for (key, value) in &request.headers {
            req = req.header(key, value);
            info!("Headers: {} : {}",key, value);
        }

        if let Some(body) = &request.body {
            let body_json = serde_json::to_string(body)?;
            info!("Body: {}", &body_json);
            req = req.body(body_json);
        }

        let response = req.send().await?;
        if !response.status().is_success() {
            error!("Request failed with status: {}", response.status());
            return Err(format!("Request failed with status: {}", response.status()).into());
        }
        
        let response_text = response.text().await?;
        info!("Response body: {}", response_text);
    } else if request.method == "GET" {
        let mut req = client.get(&request.url);

        for (key, value) in &request.headers {
            req = req.header(key, value);
        }

        if let Some(body) = &request.body {
            let body_json = serde_json::to_string(body)?;
            req = req.body(body_json);
        }

        let response = req.send().await?;
        if !response.status().is_success() {
            error!("Request failed with status: {}", response.status());
            return Err(format!("Request failed with status: {}", response.status()).into());
        }
        
        let response_text = response.text().await?;
        info!("Response body: {}", response_text);
    } else if request.method == "DELETE" {
        let mut req = client.delete(&request.url);

        for (key, value) in &request.headers {
            req = req.header(key, value);
        }

        if let Some(body) = &request.body {
            let body_json = serde_json::to_string(body)?;
            req = req.body(body_json);
        }

        let response = req.send().await?;
        if !response.status().is_success() {
            error!("Request failed with status: {}", response.status());
            return Err(format!("Request failed with status: {}", response.status()).into());
        }
        
        let response_text = response.text().await?;
        info!("Response body: {}", response_text);
    } else {
        error!("Couldn't make request, method was neither POST, nor GET, nor DELETE.");
        return Err("Couldn't make request, method was neither POST, nor GET, nor DELETE.".into());
    }

    Ok(())
}