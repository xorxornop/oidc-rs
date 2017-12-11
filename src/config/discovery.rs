/// Configuration for discovery.
#[derive(Debug)]
pub(crate) struct DiscoveryConfig {
    /// Show endpoints.
    show_endpoints: bool,

    /// Show signing keys.
    show_key_set: bool,

    /// Show identity scopes.
    show_identity_scopes: bool,

    /// Show API scopes.
    show_api_scopes: bool,

    /// Show identity claims.
    show_claims: bool,

    /// Show response types.
    show_response_types: bool,

    /// Show response modes.
    show_response_modes: bool,

    /// Show standard grant types.
    show_grant_types: bool,

    /// Show custom grant types.
    show_extension_grant_types: bool,

    /// Show token endpoint authentication methods.
    show_token_endpoint_authentication_methods: bool,

    /// Turn relative paths that start with ~/ into absolute paths.
    expand_relative_paths_in_custom_entries: bool,

    /// The maxage value of the cache control header (in seconds) of the HTTP response.
    /// This gives clients a hint how often they should refresh their cached copy of the discovery document (defaults to one hour).
    response_cache_interval: Option<u32>,
    
    /// Custom entries in the discovery document.
    custom_entries: HashMap<String, String>,
}