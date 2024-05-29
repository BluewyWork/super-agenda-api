pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
   UserNotFound,

   MongoDB(mongodb::error::Error),
}

impl From<mongodb::error::Error> for Error {
   fn from(err: mongodb::error::Error) -> Self {
      Self::MongoDB(err)
   }
}
