#[macro_use] extern crate failure;
use self::error::LoxError;

mod client;
mod error;
mod protos;

pub fn connect<'a>(name: &str, address: &str) -> Result<client::Client, LoxError> {
    let cl = client::Client::new(name, address);
    match cl {
        Ok(client) => Ok(client),
        Err(e) => Err(e),
    }
}

