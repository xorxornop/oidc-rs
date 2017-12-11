use chrono::{DateTime, Utc};

/// Configuration for caching.
#[derive(Debug)]
pub(crate) struct CachingConfig {
    /// The client store expiration.
    client_store_expiration: DateTime<Utc>,

    /// The resource/scope store expiration.
    resource_store_expiration: DateTime<Utc>,

    /// The CORS origin expiration.
    cors_expiration: DateTime<Utc>,
}