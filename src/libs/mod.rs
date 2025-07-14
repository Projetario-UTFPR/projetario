use chrono::{DateTime, Utc};

pub mod actix;
pub mod inertia;
pub mod sqlx;
pub mod validator;

pub type UtcDateTime = DateTime<Utc>;
