#[derive(Debug , PartialEq , Eq)]
pub enum Error {
    HomeDirectoryNotFound,
    CannotCreateDirectory,
    CannotCreateFile,
    CannotWriteToFile,
    SerializationError,
    SerializationTOMLError,
    CannotReadFile,
    StartFunctionNotSet,
    ExecuteFunctionNotSet,
    ConfigNotFound,
    ReqwestError,
    BuilderCloneError,
    CacheIsNotAFile,
    StateIsUnset,
    InvalidHexColorCode,
    PathWithNoParent
}

