mod authentication;
mod caching;
mod cors;
mod discovery;
mod endpoints;
mod events;
mod input_length;
mod user_interaction;

use authentication;
use caching;
use cors;
use discovery;
use endpoints;
use events;
use input_length;
use user_interaction;

use std::option::Option;

/// Top-level configuration for the OIDC-RS service
#[derive(Debug)]
pub(crate) struct ServiceConfig {
    /// The unique name of this server instance, e.g. https://myissuer.com.
    /// If not set, the issuer name is inferred from the request(s).
    issuer_url: Option<String>,

    /// The origin of this server instance, e.g. https://myorigin.com.
    /// If not set, the origin is inferred from the request(s).
    public_origin: Option<String>,

    /// The authentication configuration.
    authentication: AuthenticationConfig,

    /// The caching configuration.
    caching: CachingConfig,

    /// The CORS configuration.
    cors: CorsConfig,

    /// The discovery configuration.
    discovery: DiscoveryConfig,

    /// The endpoints configuration.
    endpoints: EndpointsConfig,

    /// The events configuration.
    events: EventsConfig,

    /// The input length restrictions configuration.
    input_length_restrictions: InputLengthRestrictionsConfig,

    /// The user interaction configuration.
    user_interaction: UserInteractionConfig,
}