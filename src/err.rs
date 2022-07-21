use std::fmt;

pub enum CatError {
    EIO,
}

impl fmt::Debug for CatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for CatError {
    fn from(_f: std::io::Error) -> Self {
        CatError::EIO
    }
}

pub type Result<T> = std::result::Result<T, CatError>;
