#[derive(Debug , PartialEq)]
pub enum Error {
    HomeDirectoryNotFound,
    CannotCreateDirectory,
    CannotCreateFile,
    CannotWriteToFile,
    SerializationError,
    SerializationTOMLError,
    CannotReadFile,
    ConfigNotFound,
    CacheIsNotAFile,
    PathWithNoParent
}

