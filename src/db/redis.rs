use redis::{AsyncCommands, Client};

#[derive(Clone)]
pub struct RedisDb {
    client: &'static Client,
}

impl RedisDb {
    pub fn default(client: &'static Client) -> Self {
        Self { client: client }
    }

    /// Cache a short_code → long_url with optional TTL (seconds)
    pub async fn cache_url(
        &self,
        short_code: &str,
        long_url: &str,
        ttl_seconds: Option<u64>,
    ) -> redis::RedisResult<()> {
        // Multiplexed async connection (recommended for async use)
        println!("Caching URL: {} -> {}", short_code, long_url);
        let conn = self
            .client
            .get_multiplexed_async_connection()
            .await;
        if let Err(e) = conn {
            eprintln!("Error connecting to Redis: {}", e);
            return Err(e.into());
        }
        let mut conn = conn.unwrap();
        println!("Connection established");
        match ttl_seconds {
            Some(ttl) => {
                let _: () = conn.set_ex(short_code, long_url, ttl).await?;
            }
            None => {
                let _: () = conn.set(short_code, long_url).await?;
            }
        }

        Ok(())
    }

    /// Fetch cached URL
    pub async fn get_url(&self, short_code: &str) -> redis::RedisResult<Option<String>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let val: Option<String> = conn.get(short_code).await?;
        Ok(val)
    }
}
