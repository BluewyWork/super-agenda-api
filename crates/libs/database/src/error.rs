use std::fmt::Display;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
   UnableToCreateUser,
   UnableToFindUser,
   UnableToFindUserData,
   UnableToFindTask,

   MongoDB(mongodb::error::Error),
   MongoDBBson(mongodb::bson::ser::Error),
   MongoDBOID(mongodb::bson::oid::Error),
}

impl Display for Error {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let message = match self {
         Self::UnableToCreateUser => String::from("unable to create user"),
         Self::UnableToFindUser => String::from("unable to find user"),
         Self::UnableToFindUserData => String::from("unable to find task group"),
         Self::UnableToFindTask => String::from("unable to find task"),

         Self::MongoDB(err) => err.to_string(),
         Self::MongoDBBson(err) => err.to_string(),
         Self::MongoDBOID(err) => err.to_string(),
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

impl From<mongodb::bson::ser::Error> for Error {
   fn from(err: mongodb::bson::ser::Error) -> Self {
      Self::MongoDBBson(err)
   }
}

impl From<mongodb::bson::oid::Error> for Error {
   fn from(err: mongodb::bson::oid::Error) -> Self {
      Self::MongoDBOID(err)
   }
}
