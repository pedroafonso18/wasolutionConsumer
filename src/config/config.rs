use dotenvy;
use std::env;
use log;

pub struct DotEnv {
    pub rabbit_url: String,
    pub db_url: String,
    pub redis_url: String,
}

pub fn load_dotenv() -> Result<DotEnv, Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    
    let rabbit_url = env::var("RABBIT_URL")
        .map_err(|e| format!("Failed to get RABBIT_URL: {}", e))?;
    let db_url = env::var("DB_URL")
        .map_err(|e| format!("Failed to get DB_URL: {}", e))?;
    let redis_url = env::var("REDIS_URL")
        .map_err(|e| format!("Failed to get REDIS_URL: {}", e))?;
    
    log::info!(".ENV Vars loaded successfully! Returning them now...");

    Ok(DotEnv {
        rabbit_url,
        db_url,
        redis_url
    })
}