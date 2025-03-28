pub mod gridsquare;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid code")]
    InvalidCode,
}
