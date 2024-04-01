use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
   pub username: String,
   pub display_name: String,
   pub password: String,
   pub email: Option<String>,
   pub recovery_email: Option<String>,
   pub phone: Option<Phone>,
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
