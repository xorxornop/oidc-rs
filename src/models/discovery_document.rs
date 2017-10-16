use std::option::Option;
use std::vec::Vec;

pub struct DiscoveryDocument {
    issuer: String,
    jwks_uri: String,
    authorization_endpoint: String,
    token_endpoint: String,
    userinfo_endpoint: String,
    end_session_endpoint: String,
    check_session_iframe: String,
    revocation_endpoint: String,
    introspection_endpoint: String,
    frontchannel_logout_supported: Option<bool>,
    frontchannel_logout_session_supported: Option<bool>,
    scopes_supported: Vec<String>,
    claims_supported: Vec<String>,
    response_types_supported: Vec<String>,
    response_modes_supported: Vec<String>,
    grant_types_supported: Vec<String>,
    subject_types_supported: Vec<String>,
    id_token_signing_alg_values_supported: Vec<String>,
    token_endpoint_auth_methods_supported: Vec<String>,
    code_challenge_methods_supported: Vec<String>
}