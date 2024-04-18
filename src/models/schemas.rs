use mongodb::bson::{oid::ObjectId, serde_helpers::serialize_bson_datetime_as_rfc3339_string, Bson, DateTime, Document};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::response::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
   pub id: ObjectId,
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
         id: ObjectId::new(),
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

#[derive(Deserialize, Serialize)]
pub struct TaskGroup {
   pub id: ObjectId,
   pub owner: ObjectId,
   pub list: Vec<Task>,
}

#[derive(Deserialize, Serialize)]
pub struct Task {
   pub id: ObjectId,
   pub title: Option<String>,
   pub description: Option<String>,
   pub status: Option<TaskStatus>,
   pub priority: Option<TaskPriority>,
   pub schedule: Option<TaskSchedule>,
}

impl From<Task> for Bson {
    fn from(task: Task) -> Self {
        let mut doc = Document::new();
        doc.insert("_id", task.id);
        doc.insert("title", task.title);
        doc.insert("description", task.description);
        doc.insert("status", task.status);
        doc.insert("priority", task.priority);
        doc.insert("schedule", task.schedule);
        Bson::Document(doc)
    }
}

#[derive(Deserialize, Serialize)]
pub enum TaskStatus {
   NotStarted,
   OnGoing,
   Completed,
}

impl From<TaskStatus> for Bson {
    fn from(status: TaskStatus) -> Self {
        match status {
            TaskStatus::NotStarted => Bson::String("NotStarted".to_string()),
            TaskStatus::OnGoing => Bson::String("OnGoing".to_string()),
            TaskStatus::Completed => Bson::String("Completed".to_string()),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum TaskPriority {
   Low,
   Normal,
   High,
}

impl From<TaskPriority> for Bson {
    fn from(priority: TaskPriority) -> Self {
        match priority {
            TaskPriority::Low => Bson::String("Low".to_string()),
            TaskPriority::Normal => Bson::String("Normal".to_string()),
            TaskPriority::High => Bson::String("High".to_string()),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct TaskSchedule {
   pub start: Option<DateTime>,
   pub end: Option<DateTime>,
   pub estimated_duration_minutes: Option<i64>,
}

impl From<TaskSchedule> for Bson {
    fn from(schedule: TaskSchedule) -> Self {
        let mut doc = Document::new();
        if let Some(start) = schedule.start {
            doc.insert("start", start);
        }
        if let Some(end) = schedule.end {
            doc.insert("end", end);
        }
        if let Some(estimated_duration_minutes) = schedule.estimated_duration_minutes {
            doc.insert("estimated_duration_minutes", estimated_duration_minutes);
        }
        Bson::Document(doc)
    }
}
