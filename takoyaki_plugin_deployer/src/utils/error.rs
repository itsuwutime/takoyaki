#[derive(Debug)]
pub enum Error {
    CannotListenOnAddress,
    HandshakeError,
    CannotParseRecievedData,
}

