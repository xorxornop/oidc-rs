use std::vec::Vec;

use super::resource::Resource;

/// Models a user identity resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityResource {
    /// Indicates if this resource is enabled. Defaults to true.
    enabled: bool,

    /// The unique name of the resource.
    name: String,

    /// Display name of the resource.
    display_name: String,

    /// Description of the resource.
    description: String,

    /// List of accociated user claims that should be included when this resource is requested.
    user_claims: Vec<String>,

    /// Specifies whether the user can de-select the scope on the consent screen (if the consent screen wants to implement such a feature). Defaults to false.
    required: bool,

    /// Specifies whether the consent screen will emphasize this scope (if the consent screen wants to implement such a feature). 
    /// Use this setting for sensitive or important scopes. Defaults to false.
    emphasise: bool,

    /// Specifies whether this scope is shown in the discovery document. Defaults to true.
    show_in_discovery_document: bool
}

impl Resource for IdentityResource {
    fn new(name: String) -> IdentityResource {
        IdentityResource::new_with_claims(name, vec![])
    }

    fn new_with_claims(name: String, user_claims: Vec<String>) -> IdentityResource {
        IdentityResource {
            name,
            enabled: true,
            display_name: "".to_owned(),
            description: "".to_owned(),
            user_claims,

            required: false,
            emphasise: false,
            show_in_discovery_document: true
        }
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn display_name(&self) -> &String {
        &self.display_name
    }

    fn description(&self) -> &String {
        &self.description
    }

    fn user_claims(&self) -> &Vec<String>{
        &self.user_claims
    }
}

impl IdentityResource {
    fn new_openid() -> IdentityResource {
        IdentityResource {
            name: "openid".to_owned(),
            enabled: true,
            display_name: "Your user identifier".to_owned(),
            description: "".to_owned(),
            user_claims: vec!["sub".to_owned()],

            required: true,
            emphasise: false,
            show_in_discovery_document: true
        }
    }
}