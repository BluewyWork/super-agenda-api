use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
   #[error("This is an example error message.")]
   Example
}
