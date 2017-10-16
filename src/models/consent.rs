use std::vec::Vec;
use std::option::Option;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Consent {
    /// The subject identifier
    subject_id: String,

    /// The client identifier
    client_id: String,

    /// The scopes
    scopes: Vec<String>,

    /// The creation time
    creation_time: DateTime<Utc>,

    /// The expiration time
    expiration: Option<DateTime<Utc>>
}