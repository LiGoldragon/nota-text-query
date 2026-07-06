use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, ThisError)]
pub enum Error {
    #[error("rkyv encoding failed: {0}")]
    RkyvEncode(String),

    #[error("rkyv decoding failed: {0}")]
    RkyvDecode(String),
}
