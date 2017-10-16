use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The audiences
    audiences: Vec<String>,

    /// The issuer
    issuer: String,

    /// The creation time
    creation_time: DateTime<Utc>,

    /// The lifetime in seconds
    lifetime: u32,

    /// The type of token
    type_of: String,

    /// The ID of the client
    client_id: String,

    /// The access token type specified by the client.
    access_token_type: String,

    /// The claims
    claims: Vec<String>,

    /// The version
    version: u32
}

impl Token {
    pub fn subject_id() -> Vec<String> {
        unimplemented!();
    }

    pub fn scopes() -> Vec<String> {
        unimplemented!();
    }
}