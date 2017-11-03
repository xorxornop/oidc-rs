use std::collections::HashSet;
use std::hash::{Hash, Hasher};

/// Models access to an API resource
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scope {
    /// Name of the scope. This is the value a client will use to request the scope.
    name: String,
    
    /// Display name. This value will be used e.g. on the consent screen.
    display_name: String,

    /// Description. This value will be used e.g. on the consent screen.
    description: String,

    /// Specifies whether the user can de-select the scope on the consent screen. Defaults to false.
    required: bool,

    /// Specifies whether the consent screen will emphasize this scope. Use this setting for sensitive or important scopes. Defaults to false.
    emphasise: bool,

    /// Specifies whether this scope is shown in the discovery document. Defaults to true.
    show_in_discovery_document: bool,

    /// List of user-claim types that should be included in the access token.
    user_claims: HashSet<String>
}

impl Hash for Scope {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.display_name.hash(state);
        self.description.hash(state);
        self.required.hash(state);
        self.emphasise.hash(state);
        self.show_in_discovery_document.hash(state);
        
        for claim in &self.user_claims {
            claim.hash(state);
        }
    }
}