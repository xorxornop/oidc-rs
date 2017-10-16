use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
struct AuthorisationCode {
    /// The creation time
    creation_time: DateTime<Utc>,

    /// The lifetime in seconds
    lifetime: u32,

    /// The ID of the client
    client_id: String,

    /// The subject
    subject: String,

    /// Whether this code is an OpenID Connect code
    /// <c>true</c> if this instance is open identifier; otherwise, <c>false</c>
    is_openid: bool,

    /// The requested scopes
    requested_scopes: Vec<String>,

    /// The redirect URI
    redirect_uri: String,

    /// The nonce
    nonce: String,

    /// Whether consent was shown
    /// <c>true</c> if consent was shown; otherwise, <c>false</c>
    was_consent_shown: bool,

    /// The session identifier.
    session_id: String,

    /// The code challenge
    code_challenge: String,

    /// The code challenge method
    code_challenge_method: String
}