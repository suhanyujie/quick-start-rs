pub use crate::errors::Error;

pub type Result<T> = std::result::Result<T, Error>;

// Your wrapper type
pub struct W<T>(pub T);

pub use std::format as f;
