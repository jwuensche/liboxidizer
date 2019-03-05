use crate::error::LoxError;
use crate::krpc::Argument;

pub struct Vessel<'a> {
    pub client: &'a mut super::Client,
    pub vessel: std::vec::Vec<u8>,
    pub control: std::vec::Vec<u8>,
}

impl<'a> Vessel<'a> {

    pub fn new(client: &mut super::Client, vessel: std::vec::Vec<u8>) -> Vessel {
        Vessel{client: client, vessel: vessel, control: std::vec::Vec::default()}
    }

    pub fn get_control(&mut self) -> Result<std::vec::Vec<u8>, LoxError> {
        let mut arg = protobuf::RepeatedField::new();
        let mut arg1 = Argument::new();
        arg1.set_position(0);
        arg1.set_value(self.vessel.clone());
        arg.push(arg1);

        match self.client.get_parameter("SpaceCenter", "Vessel_get_Control", arg) {
            Ok(val) => {
                self.control = *val;
                Ok(self.control.clone())
            }
            Err(e) => {
                Err(LoxError::InvalidResponse{content: format!("{}", e)})
            }
        }
    }

}
