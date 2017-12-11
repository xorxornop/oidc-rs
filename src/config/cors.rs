use std::collections::HashSet;
use chrono::{DateTime, Utc};

/// Configuration for CORS.
#[derive(Debug)]
pub(crate) struct CorsConfig {
    /// The name of the cors policy.
    cors_policy_name: DateTime<Utc>,

    /// The value to be used in the preflight `Access-Control-Max-Age` response header.
    preflight_cache_duration: DateTime<Utc>,

    /// The CORS paths.
    cors_paths: HashSet<String>,
}