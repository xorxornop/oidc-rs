mod constants;

pub mod grant_type;
pub mod jwt_claim_type;
pub mod protocol_type;
pub mod token_type;

/// OpenID Connect subject types.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubjectType {
    /// global - use the native subject id
    Global,

    /// ppid - scope the subject id to the client
    Ppid
}

/// Access token types.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessTokenType {
    /// Self-contained Json Web Token
    Jwt,

    /// Reference token
    Reference
}

/// Token usage types.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenUsage {
    /// Re-use the refresh token handle
    ReUse,

    /// Issue a new refresh token handle every time
    OneTimeOnly
}

/// Token expiration types.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenExpiration {
    /// Sliding token expiration
    Sliding,

    /// Absolute token expiration
    Absolute
}