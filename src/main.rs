mod config;
mod db;
mod models;
mod services;
mod utils;

use rocket::{launch, post};
use rocket::response::Redirect;
use rocket::{Build, Rocket, get, routes, serde::Serialize, serde::json::Json};

use crate::utils::RedirectError;

#[get("/")]
fn index() -> &'static str {
    "🚀 Hello from Rocket!"
}

#[derive(Serialize)]
struct Health {
    status: String,
}

#[get("/health")]
fn health() -> Json<Health> {
    Json(Health {
        status: "ok".to_string(),
    })
}

#[get("/<short_url>")]
async fn redirect(short_url: String) -> Result<Redirect, RedirectError> {
    println!("Redirecting to: {}", short_url);
    match services::url_service::redirect_url(&short_url).await {
        Ok(long_url) => Ok(Redirect::to(long_url)),
        Err(_) => Err(RedirectError::NotFound),
    }
}

#[post("/shorten", data = "<long_url>")]
async fn shorten(long_url: String) -> Result<Json<String>, RedirectError> {
    match services::url_service::shorten_url(&long_url).await {
        Ok(short_url) => Ok(Json(short_url)),
        Err(_) => Err(RedirectError::Internal),
    }
}

#[launch]
async fn rocket() -> Rocket<Build> {
    println!("Starting Rocket...");
    dotenv::dotenv().ok();
    config::init_config().await;
    rocket::build()
        .mount("/", routes![redirect, index, health])
        .mount("/api/v1", routes![shorten, redirect])
}
