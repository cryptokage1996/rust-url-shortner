use std::{env, time::Duration};

use once_cell::sync::OnceCell;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use redis::Client;
use rocket::tokio::{self, time::sleep};

use crate::db::sqlite::SqliteDb;

pub static SQLITE_POOL: OnceCell<Pool<SqliteConnectionManager>> = OnceCell::new();
pub static REDIS_CLIENT: OnceCell<Client> = OnceCell::new();
pub static HOST_NAME: OnceCell<String> = OnceCell::new();


pub async fn init_config(){
    SQLITE_POOL.set(
        Pool::new(
            SqliteConnectionManager::file(
                "data/shortener.db"
            )
        ).unwrap()
    ).unwrap();
    let environment = env::var("ENV").unwrap_or("development".to_string());
    if environment == "development" {
        let data = format!("http://localhost:{}", env::var("PORT").unwrap());
        HOST_NAME.set(data).unwrap();
    }
    REDIS_CLIENT.set(Client::open(env::var("REDIS_URL").unwrap()).unwrap()).unwrap();
    println!("Initialized Redis client");
    let sqlite_db = SqliteDb::default(SQLITE_POOL.get().unwrap());
    sqlite_db.init_table().unwrap();
    println!("Initialized SQLite pool");
    tokio::spawn(async move {
        sqlite_db.remove_rows_created_in_last_day().unwrap();
        println!("Removed rows created in last day");
        sleep(Duration::from_secs(86400)).await;
    });
}
