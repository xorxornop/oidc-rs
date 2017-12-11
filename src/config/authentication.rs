use chrono::{DateTime, Utc};

/// Configuration for authentication, e.g. the login and logout views and behavior.
#[derive(Debug)]
pub(crate) struct AuthenticationConfig {
    /// The cookie lifetime.
    cookie_lifetime: DateTime<Utc>,

    /// Whether the cookie should be sliding or not.
    cookie_sliding_expiration: bool,

    /// Require user to be authenticated to accept parameters to end session endpoint. Defaults to false.
    require_authenticated_user_for_sign_out_message: bool,

    /// The name of the cookie used for the check session endpoint.
    cookie_name_for_check_session: String,
}

impl AuthenticationConfig {
    fn new() -> AuthenticationConfig {
        AuthenticationConfig {
            cookie_lifetime: DateTime<Utc>::new(), // Default
            cookie_sliding_expiration: false,
            require_authenticated_user_for_sign_out_message: false,
            cookie_name_for_check_session: String::from("oidc-rs")
        }
    }
}