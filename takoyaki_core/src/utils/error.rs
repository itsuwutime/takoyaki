#[derive(Debug)]
pub enum Error {
    NoStartFunctionFound,
    NoExecuteFunctionFound,
    ConfigDirNotFound,
    CannotCreateDirectory,
    NoConfigFound,
    ReadError,
    BuggedConfig,
    StateUnset,
    InvalidHexColorCode,
    ReqwestError(reqwest::Error),
    SerializeJSONError(serde_json::Error),
}
