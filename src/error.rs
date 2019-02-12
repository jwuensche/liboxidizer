use failure::Fail;

#[derive(Debug, Fail)]
pub enum LoxError {
    #[fail(display = "invalid client address: {}", address)]
    InvalidClientAddress {
        address: String,
        error: websocket::WebSocketError,
    },
    #[fail(display = "failure while sending: {}", content)]
    FailureOnSend {
        content: String,
    },
    #[fail(display = "generic error")]
    GenericError {
        content: String,
    }
}
