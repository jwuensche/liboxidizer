mod vessel;

use crate::error::LoxError;
use self::vessel::Vessel;
use crate::byteorder::{BigEndian, ReadBytesExt};
use crate::krpc::{ProcedureCall, Request, Response, Status, Argument};


pub struct Client {
    name: String,
    connection: websocket::client::sync::Client<websocket::stream::sync::TcpStream>,
}

impl<'a> Client {

    pub fn new(name: &str, address: &str) -> Result<Client, LoxError> {
        let builder = websocket::ClientBuilder::new(format!("{}{}{}", address, "/?name=", name).as_str());
        match builder {
            Ok(mut c) => {
                match c.connect_insecure() {
                    Ok(streamresult) => {
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

    fn send(&mut self, msg: &Request) -> Result<Response, LoxError>{
        let mg = msg as &protobuf::Message;
        match mg.write_to_bytes() {
            Ok(res) => {
        let ws_msg = websocket::Message::binary(res);
        let send = self.connection.send_message(&ws_msg);
        match send {
            Ok(_c) => {
                let resp = self.connection.recv_message();
                match resp {
                    Ok(resp_content) => {
                        let resp_msg = websocket::Message::from(resp_content);
                        let deserialized = protobuf::parse_from_bytes(&*resp_msg.payload);
                        match deserialized {
                            Ok(c) => {
                                Ok(c)
                            }
                            Err(e) => {
                                Err(LoxError::InvalidResponse{content: format!("{}", e)})
                            }
                        }
                    }
                    Err(e) => {
                        Err(LoxError::FailureOnReceive{content: format!("{}", e)})
                    }
                }
            },
            Err(e) => {
                Err(LoxError::FailureOnSend{content: format!("{}", e)})
            }
        }
            }
            Err(_e) => {
                Err(LoxError::InvalidRequest{})
            }
        }

    }

    fn create_req(&self, service: &str, procedure: &str, arg: protobuf::RepeatedField<Argument>) -> Request {
        let mut req = Request::new();
        let mut call = ProcedureCall::new();

        call.service = service.to_string();
        call.procedure = procedure.to_string();
        call.arguments = arg;
        req.calls.push(call);

        return req
    }

    pub fn get_status(&mut self) -> Result<Status, LoxError> {
        let req = self.create_req("KRPC", "GetStatus", protobuf::RepeatedField::default());
        match self.send(&req) {
            Ok(result) => {
                match protobuf::parse_from_bytes(&result.results[0].value) {
                    Ok(c) => {
                        Ok(c)
                    }
                    Err(e) => {
                        Err(LoxError::InvalidResponse{content: format!("{}", e)})
                    }
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    fn get_parameter(&mut self, service: &str, procedure: &str, arg: protobuf::RepeatedField<Argument>) -> Result<Box<std::vec::Vec<u8>>, LoxError> {
        let req = self.create_req(service, procedure, arg);
        match self.send(&req) {
            Ok(mut result) => {
                Ok(Box::new(result.results[0].take_value()))
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn get_client_name(&mut self) -> Result<Box<std::string::String>, LoxError> {
        match self.get_parameter("KRPC", "GetClientName", protobuf::RepeatedField::default()) {
            Ok(val) => {
                match String::from_utf8(*val) {
                    Ok(c) => {
                        Ok(Box::new(c))
                    }
                    Err(e) => {
                        Err(LoxError::UnknownBytes{content: format!("{}", e)})
                    }
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn get_client_id(&mut self) -> Result<Box<std::string::String>, LoxError> {
        match self.get_parameter("KRPC", "GetClientID", protobuf::RepeatedField::default()) {
            Ok(val) => {
                let mut rdr = std::io::Cursor::new(*val);
                match rdr.read_u16::<BigEndian>(){
                    Ok(c) => {
                        Ok(Box::new(c.to_string()))
                    }
                    Err(e) => {
                        Err(LoxError::UnknownBytes{content: format!("{}", e)})
                    }
                }
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    pub fn get_active_vessel(&mut self) -> Result<Vessel, LoxError> {
        match self.get_parameter("KRPC", "get_ActiveVessel", protobuf::RepeatedField::default()) {
            Ok(val) => {
                Ok(Vessel::new(self, *val))
            }
            Err(_e) => {
                Err(LoxError::GenericError{content: "Foo".to_string()})
            }
        }
    }

    pub fn save(&mut self, name: &str) -> Result<bool, LoxError> {
        let mut arg = protobuf::RepeatedField::new();
        let mut arg1 = Argument::new();
        arg1.set_position(0);
        arg1.set_value((*name.as_bytes()).to_vec());
        arg.push(arg1);
        match self.get_parameter("SpaceCenter", "Save", arg) {
            Ok(_v) => {
                Ok(true)
            }
            Err(e) => {
                Err(LoxError::GenericError{content: format!("{}", e)})
            }
        }
    }

}
