use self::{error::Error, success::Success};

pub type Result = core::result::Result<Success, Error>;

pub mod error;
pub mod success;
