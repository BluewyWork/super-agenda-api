use mongodb::bson::DateTime;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::response::{self, Error};

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
         return Err(response::Error::InvalidUsername);
      }

      if password.len() < 5 {
         return Err(response::Error::InvalidPassword);
      }

      if let Some(email) = &email {
         let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
         if !email_regex.is_match(email) {
            return Err(response::Error::InvalidEmail);
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Phone {
   pub country_code: u16,
   pub number: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
   pub object_id: String,
   pub title: Option<String>,
   pub description: Option<String>,
   pub status: Status,
   pub priority: Priority,
   pub tags: Option<Vec<String>>,
   pub planning: Option<Planning>,
   pub metrics: Option<Metrics>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Status {
   NotStarted,
   Ongoing,
   Completed,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Priority {
   High,
   Normal,
   Low,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Planning {
   initiation: Option<DateTime>,
   deadline: Option<DateTime>,
   estimated_duration_in_minutes: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metrics {
   date_time_when_status_is_started: Option<DateTime>,
   date_time_when_status_is_completed: Option<DateTime>,
}
