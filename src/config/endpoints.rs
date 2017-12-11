/// Configuration for endpoints, e.g. which are enabled or disabled.
#[derive(Debug)]
pub(crate) struct EndpointsConfig {
    /// Whether the authorize endpoint is enabled.
    enable_authorize_endpoint: bool,

    /// Whether the token endpoint is enabled.
    enable_token_endpoint: bool,

    /// Whether the userinfo endpoint is enabled.
    enable_user_info_endpoint: bool,

    /// Whether the discovery endpoint is enabled.
    enable_discovery_endpoint: bool,

    /// Whether the end session endpoint is enabled.
    enable_end_session_endpoint: bool,

    /// Whether the check sesison endpoint is enabled.
    enable_check_session_endpoint: bool,

    /// Whether the token revocation endpoint is enabled.
    enable_token_revocation_endpoint: bool,

    /// Whether the introspection endpoint is enabled.
    enable_introspection_endpoint: bool,
}
