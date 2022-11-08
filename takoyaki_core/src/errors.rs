#[derive(Debug)]
pub enum Errors {
    NoStartFunctionFound,
    NoExecuteFunctionFound,
    ConfigDirNotFound,
    CannotCreateDirectory,
    NoConfigFound,
    ReadError,
    BuggedConfig,
    StateUnset,
    ReqwestError(reqwest::Error),
    SerializeJSONError(serde_json::Error),
}
