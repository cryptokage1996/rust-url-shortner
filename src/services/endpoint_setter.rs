use std::error::Error;

use crate::config::HOST_NAME;

pub fn set_endpoint(endpoint: &str) -> Result<(), Box<dyn Error>> {
    println!("Setting endpoint: {}", endpoint);
    let old_endpoint = HOST_NAME.get();
    if old_endpoint.is_some() {
        return Err("Endpoint already set".into());
    }
    HOST_NAME.set(endpoint.to_string()).unwrap();
    Ok(())
}