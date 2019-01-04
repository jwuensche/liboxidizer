use crate::error::LoxError;

pub struct Client {
    name: String,
    connection: websocket::client::sync::Client<websocket::stream::sync::TcpStream>,
}

impl Client {
    pub fn new(name: &str, address: &str) -> Result<Client, LoxError> {
        let builder = websocket::ClientBuilder::new(address);
        match builder {
            Ok(mut c) => {
                match c.connect_insecure() {
                    Ok(streamresult) => {
                        println!("It works!");
                        let cl = Client{name: String::from(name), connection: streamresult};
                        Ok(cl)
                    },
                    Err(_e) => {
                        println!("Oh no, while creating the client");
                        Err(LoxError::InvalidClientAddress{address: address.to_string()})
                    }
                }
            },
            Err(_e) => {
                println!("Oh no!");
                Err(LoxError::InvalidClientAddress{address: address.to_string()})
            },
        }
    }

    pub fn send(&self) {
    }
}
