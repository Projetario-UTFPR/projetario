use chrono::{DateTime, Utc};

pub mod actix;
pub mod validator;

pub type UtcDateTime = DateTime<Utc>;
