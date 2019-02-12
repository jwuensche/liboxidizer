use crate::error::LoxError;
use crate::protos;

pub struct Client {
    name: String,
    connection: websocket::client::sync::Client<websocket::stream::sync::TcpStream>,
}

impl Client {
    pub fn new(name: &str, address: &str) -> Result<Client, LoxError> {
        let builder = websocket::ClientBuilder::new(format!("{}{}{}", address, "/?name=", name).as_str());
        match builder {
            Ok(mut c) => {
                match c.connect_insecure() {
                    Ok(streamresult) => {
                        println!("It works!");
                        let cl = Client{name: String::from(name), connection: streamresult};
                        Ok(cl)
                    },
                    Err(e) => {
                        Err(LoxError::InvalidClientAddress{address: address.to_string(), error: e})
                    }
                }
            },
            Err(_e) => {
                Err(LoxError::GenericError{content: "Error while creating client".to_string()})
            },
        }
    }

    pub fn send(&self, msg: protos::Request) {
        //let vec: Vec<u8> = Vec::new();
        //let stream = protobuf::CodedOutputStream::vec(&mut vec);

        //msg.write_to_with_cached_sizes(stream);
        //stream.flush();

        //let ws_msg = websocket::Message::binary(vec);
        //self.connection.send_message(&ws_msg);
    }

    pub fn get_status(&self) {
        //let mut foo = protos::Request::new();
        //let bar = protos::ProcedureCall::new();
        //bar.service = String::from("KRPC");
        //bar.procedure = String::from("GetStatus");
        //foo.calls.push(bar);
    }
}
