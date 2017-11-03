use std::collections::{HashSet, HashMap};
use std::option::Option;

use super::claim::*;
use super::reference::*;
use super::secret::*;


/// Models an OpenID Connect or OAuth2 client
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Client {
    /// Specifies if client is enabled (defaults to <c>true</c>)
    enabled: bool,

    /// Unique ID of the client
    client_id: String,

    /// The protocol type.
    protocol_type: String,

    /// Client secrets - only relevant for flows that require a secret
    client_secrets: HashSet<Secret>,

    /// If set to false, no client secret is needed to request tokens at the token endpoint (defaults to <c>true</c>)
    require_client_secret: bool,

    /// Client display name (used for logging and consent screen)
    client_name: String,

    /// URI to further information about client (used on consent screen)
    client_uri: String,

    /// URI to client logo (used on consent screen)
    logo_uri: String,

    /// Specifies whether a consent screen is required (defaults to <c>true</c>)
    require_consent: bool,

    /// Specifies whether user can choose to store consent decisions (defaults to <c>true</c>)
    allow_remember_consent: bool,

    /// Specifies the allowed grant types (legal combinations of AuthorizationCode, Implicit, Hybrid, ResourceOwner, ClientCredentials). Defaults to Implicit.
    allowed_grant_types: HashSet<String>,

    /// Specifies whether a proof key is required for authorization code based token requests (defaults to <c>false</c>).
    require_pkce: bool,

    /// Specifies whether a proof key can be sent using plain method (not recommended and defaults to <c>false</c>.)
    allow_plain_text_pkce: bool,

    /// Controls whether access tokens are transmitted via the browser for this client (defaults to <c>false</c>).
    /// This can prevent accidental leakage of access tokens when multiple response types are allowed.
    allow_access_tokens_via_browser: bool,

    /// Specifies allowed URIs to return tokens or authorization codes to
    redirect_uris: HashSet<String>,

    /// Specifies allowed URIs to redirect to after logout
    post_logout_redirect_uris: HashSet<String>,

    /// Specifies logout URI at client for HTTP front-channel based logout.
    front_channel_logout_uri: String,

    /// Specifies is the user's session id should be sent to the FrontChannelLogoutUri. Defaults to <c>true</c>.
    front_channel_logout_session_required: bool,

    /// Specifies logout URI at client for HTTP back-channel based logout.
    back_channel_logout_uri: String,

    /// Specifies is the user's session id should be sent to the BackChannelLogoutUri. Defaults to <c>true</c>.
    back_channel_logout_session_required: bool,

    /// Gets or sets a value indicating whether [allow offline access]. Defaults to <c>false</c>.
    allow_offline_access: bool,

    /// Specifies the api scopes that the client is allowed to request. If empty, the client can't access any scope
    allowed_scopes: HashSet<String>,

    /// When requesting both an id token and access token, should the user claims always be added to the id token instead of requring the client to use the userinfo endpoint.
    /// Defaults to <c>false</c>.
    always_include_user_claims_in_id_token: bool,

    /// Lifetime of identity token in seconds (defaults to 300 seconds / 5 minutes)
    identity_token_lifetime: u32,

    /// Lifetime of access token in seconds (defaults to 3600 seconds / 1 hour)
    access_token_lifetime: u32,

    /// Lifetime of authorization code in seconds (defaults to 300 seconds / 5 minutes)
    authorization_code_lifetime: u32,

    /// Maximum lifetime of a refresh token in seconds. Defaults to 2592000 seconds / 30 days
    absolute_refresh_token_lifetime: u32,

    /// Sliding lifetime of a refresh token in seconds. Defaults to 1296000 seconds / 15 days
    sliding_refresh_token_lifetime: u32,

    /// Lifetime of a user consent in seconds. Defaults to null (no expiration)
    consent_lifetime: Option<u32>,

    /// ReUse: the refresh token handle will stay the same when refreshing tokens
    /// OneTime: the refresh token handle will be updated when refreshing tokens
    refresh_token_usage: TokenUsage,

    /// Gets or sets a value indicating whether the access token (and its claims) should be updated on a refresh token request.
    /// Defaults to <c>false</c>.
    update_access_token_claims_on_refresh: bool,

    /// Absolute: the refresh token will expire on a fixed point in time (specified by the AbsoluteRefreshTokenLifetime)
    /// Sliding: when refreshing the token, the lifetime of the refresh token will be renewed (by the amount specified in SlidingRefreshTokenLifetime). The lifetime will not exceed AbsoluteRefreshTokenLifetime.
    refresh_token_expiration: TokenExpiration,

    /// Specifies whether the access token is a reference token or a self contained JWT token (defaults to Jwt).
    access_token_type: AccessTokenType,

    /// A value indicating whether the local login is allowed for this client. Defaults to <c>true</c>.
    enable_local_login: bool,

    /// Specifies which external IdPs can be used with this client (if list is empty all IdPs are allowed). Defaults to empty.
    identity_provider_restrictions: HashSet<String>,

    /// A value indicating whether JWT access tokens should include an identifier. Defaults to <c>false</c>.
    include_jwt_id: bool,

    /// Claims for the client (will be included in access tokens).
    claims: HashSet<Claim>,

    /// A value indicating whether client claims should be always included in the access tokens - or only for client credentials flow.
    /// Defaults to <c>false</c>
    always_send_client_claims: bool,

    /// A value to prefix on client claim types. Defaults to <c>client_</c>.
    client_claims_prefix: String,

    /// A salt value used in pair-wise subjectId generation for users of this client.
    pair_wise_subject_salt: String,

    /// The allowed CORS origins for JavaScript clients.
    allowed_cors_origins: HashSet<String>,

    /// The custom properties for the client.
    properties: HashMap<String, String>
}

impl Client {
    pub fn new() -> Client {
        Client {
            enabled: true,
            client_id: String::new(),
            protocol_type: protocol_type::ProtocolType::OpenIdConnect.to_string(),
            client_secrets: HashSet::default(),
            require_client_secret: true,
            client_name: String::new(),
            client_uri: String::new(),
            logo_uri: String::new(),
            require_consent: true,
            allow_remember_consent: true,
            allowed_grant_types: HashSet::default(),
            require_pkce: false,
            allow_plain_text_pkce: false,
            allow_access_tokens_via_browser: false,
            redirect_uris: HashSet::default(),
            post_logout_redirect_uris: HashSet::default(),
            front_channel_logout_uri: String::new(),
            front_channel_logout_session_required: true,
            back_channel_logout_uri: String::new(),
            back_channel_logout_session_required: true,
            allow_offline_access: false,
            allowed_scopes: HashSet::default(),
            always_include_user_claims_in_id_token: false,
            identity_token_lifetime: 300,
            access_token_lifetime: 3600,
            authorization_code_lifetime: 300,
            absolute_refresh_token_lifetime: 2592000,
            sliding_refresh_token_lifetime: 1296000,
            consent_lifetime: Option::None,
            refresh_token_usage: TokenUsage::OneTimeOnly,
            update_access_token_claims_on_refresh: false,
            refresh_token_expiration: TokenExpiration::Absolute,
            access_token_type: AccessTokenType::Jwt,
            enable_local_login: true,
            identity_provider_restrictions: HashSet::default(),
            include_jwt_id: false,
            claims: HashSet::default(),
            always_send_client_claims: false,
            client_claims_prefix: String::from("client_",),
            pair_wise_subject_salt: String::new(),
            allowed_cors_origins: HashSet::default(),
            properties: HashMap::default()
        }
    }

    fn validate_grant_types(grant_types: &Vec<&str>) {
        unimplemented!();
    }
}