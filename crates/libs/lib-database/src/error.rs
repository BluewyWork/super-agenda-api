use std::fmt::Display;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
   UnableToCreateUser,
   UnableToFindUser,
   MongoDB(mongodb::error::Error)
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       let message = match self {
          Self::UnableToCreateUser => String::from("unable to create user"),
          Self::UnableToFindUser => String::from("unable to find user"),
          Self::MongoDB(err) => err.to_string()
       };

       write!(f, "ERROR => {message}")
    }
}

impl std::error::Error for Error {}

impl From<mongodb::error::Error> for Error {
    fn from(err: mongodb::error::Error) -> Self {
       Self::MongoDB(err)
    }
}
