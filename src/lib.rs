#[macro_use] extern crate failure;
use self::error::LoxError;

mod client;
mod error;

pub fn connect<'a>(address: &str) -> Result<client::Client, LoxError> {
    let cl = client::Client::new("liboxidizer", address);
    match cl {
        Ok(client) => Ok(client),
        Err(e) => Err(e),
    }
}

