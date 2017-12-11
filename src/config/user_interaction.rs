/// Configuration for user interaction, e.g. the user interface.
#[derive(Debug)]
pub(crate) struct UserInteractionConfig {
    /// The login URL. If a local URL, the value must start with a leading slash.
    login_url: String,

    /// The login return URL parameter.
    login_return_url_parameter: String,

    /// The logout URL. If a local URL, the value must start with a leading slash.
    logout_url: String,

    /// The logout identifier parameter.
    logout_id_parameter: String,

    /// The consent URL. If a local URL, the value must start with a leading slash.
    consent_url: String,

    /// The consent return URL parameter.
    consent_return_url_parameter: String,

    /// The error URL. If a local URL, the value must start with a leading slash.
    error_url: String,

    /// The error identifier parameter.
    error_id_parameter: String,

    /// The custom redirect return URL parameter.
    custom_redirect_return_url_parameter: String,

    /// The cookie message threshold. This limits how many cookies are created, and older ones will be purged.
    cookie_message_threshold: u32,
}