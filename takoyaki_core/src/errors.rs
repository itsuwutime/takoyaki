#[derive(Debug , PartialEq)]
pub enum Error {
    HomeDirectoryNotFound,
    CannotCreateDirectory,
    CannotCreateFile,
    CannotWriteToFile,
    SerializationError,
    CacheIsNotAFile,
    PathWithNoParent
}

