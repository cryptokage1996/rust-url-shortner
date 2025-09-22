use std::error::Error;

use nanoid::nanoid;

use crate::config::{HOST_NAME, REDIS_CLIENT, SQLITE_POOL};
use crate::db::redis::RedisDb;
use crate::db::sqlite::SqliteDb;
use url::Url;

const TTL_SECONDS: u64 = 60 * 60; // 1 day

pub async fn shorten_url(long_url: &str) -> Result<String, Box<dyn Error>> {
    println!("Shortening URL: {}", long_url);
    let short_code = nanoid!(6);
    let short_url = format!("{}/{}", HOST_NAME.get().unwrap(), short_code);
    println!("Short URL: {:?}", REDIS_CLIENT.get());
    let redis_db = RedisDb::default(REDIS_CLIENT.get().unwrap());
    redis_db
        .cache_url(short_code.as_str(), long_url, Some(TTL_SECONDS))
        .await?;
    let sqlite_db = SqliteDb::default(SQLITE_POOL.get().unwrap());
    let data = sqlite_db.create_short_url(short_code.as_str(), long_url);
    if let Err(e) = data {
        eprintln!("Error creating short URL: {}", e);
        return Err(e.into());
    }

    Ok(short_url)
}
pub async fn redirect_url(short_code: &str) -> Result<String, Box<dyn Error>> {
    let redis_db = RedisDb::default(REDIS_CLIENT.get().unwrap());
    let long_url = redis_db.get_url(short_code).await?;

    if long_url.is_some() {
        // Remove leading/trailing quotes
        let trimmed = long_url.as_ref().unwrap().trim_matches('"');
        let uri = Url::parse(trimmed)?;
        return Ok(uri.to_string());
    }
    let sqlite_db = SqliteDb::default(SQLITE_POOL.get().unwrap());
    let long_url_data = sqlite_db.get_long_url(short_code)?;
    if long_url_data.is_none() {
        return Err("Short code not found".into());
    }
    let data = long_url_data.unwrap();
    redis_db
        .cache_url(short_code, &data.long_url, Some(TTL_SECONDS))
        .await?;
    return Ok(data.long_url);
}
