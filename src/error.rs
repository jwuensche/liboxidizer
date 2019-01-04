use failure::Fail;

#[derive(Debug, Fail)]
pub enum LoxError {
    #[fail(display = "invalid client address: {}", address)]
    InvalidClientAddress {
        address: String,
    },
}
