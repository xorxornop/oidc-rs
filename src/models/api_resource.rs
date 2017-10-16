use std::collections::HashSet;

use super::resource::Resource;
use super::secret::Secret;
use super::scope::Scope;

/// Models a web API resource.
#[derive(Debug)]
pub struct ApiResource {
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
    
    /// The API secret is used for the introspection endpoint. The API can authenticate with introspection using the API name and secret.
    api_secrets: HashSet<Secret>,

    /// An API must have at least one scope. Each scope can have different settings.
    scopes: HashSet<Scope>
}

impl Resource for ApiResource {
    fn new(name: String) -> ApiResource {
        ApiResource::new_with_claims(name, vec![])
    }

    fn new_with_claims(name: String, user_claims: Vec<String>) -> ApiResource {
        ApiResource {
            name,
            enabled: true,
            display_name: "".to_owned(),
            description: "".to_owned(),
            user_claims,

            api_secrets: HashSet::new(),
            scopes: HashSet::new()
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