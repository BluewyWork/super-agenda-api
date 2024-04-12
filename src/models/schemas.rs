use mongodb::bson::{Bson, DateTime, Document};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::response::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
   pub username: String,
   pub display_name: String,
   pub hashed_password: String,
   pub email: Option<String>,
   pub recovery_email: Option<String>,
   pub phone: Option<Phone>,
}

impl User {
   pub fn from(
      username: String,
      display_name: String,
      password: String,
      email: Option<String>,
      recovery_email: Option<String>,
      phone: Option<Phone>,
   ) -> Result<User, Error> {
      let username_regex = Regex::new(r"^[a-z0-9_.]+$").unwrap();

      if !username_regex.is_match(&username) {
         return Err(Error::InvalidUsername);
      }

      if password.len() < 5 {
         return Err(Error::InvalidPassword);
      }

      if let Some(email) = &email {
         let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
         if !email_regex.is_match(email) {
            return Err(Error::InvalidEmail);
         }
      }

      Ok(User {
         username,
         display_name,
         hashed_password: password,
         email,
         recovery_email,
         phone,
      })
   }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Phone {
   pub country_code: i32,
   pub number: i64,
}


impl From<Phone> for Bson {
    fn from(phone: Phone) -> Self {
        let mut doc = Document::new();
        doc.insert("country_code", phone.country_code);
        doc.insert("number", phone.number);
        Bson::Document(doc)
    }
}
