use crate::error::LoxError;

use crate::krpc::{ProcedureCall, Request, Response, Status};

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

    pub fn send(&mut self, msg: &Request) -> Result<Response, LoxError>{
        let mg = msg as &protobuf::Message;
        let res = mg.write_to_bytes().unwrap();

        let ws_msg = websocket::Message::binary(res);
        self.connection.send_message(&ws_msg).unwrap();
        let resp = self.connection.recv_message().unwrap();
        assert!(resp.is_data());
        let resp_msg = websocket::Message::from(resp);

        let resp_content: Response = protobuf::parse_from_bytes(&*resp_msg.payload).unwrap();
        Ok(resp_content)
    }

    pub fn get_status(&mut self) -> Result<Status, LoxError> {
        let mut foo = Request::new();
        let mut bar = ProcedureCall::new();
        bar.service = String::from("KRPC");
        bar.procedure = String::from("GetStatus");
        foo.calls.push(bar);

        let result = self.send(&foo).unwrap();
        let status: Status = protobuf::parse_from_bytes(&result.results[0].value).unwrap();
        Ok(status)
    }
}
