use std::error::Error;

use url::Url;

use crate::config::HOST_NAME;

pub fn set_endpoint(endpoint: &str) -> Result<(), Box<dyn Error>> {
    println!("Setting endpoint: {}", endpoint);
    let old_endpoint = HOST_NAME.get();
    if old_endpoint.is_some() {
        return Err("Endpoint already set".into());
    }
    let trimmed = endpoint.trim_matches('"');
    let uri = Url::parse(trimmed)?;
    HOST_NAME.set(uri.to_string()).unwrap();
    Ok(())
}
