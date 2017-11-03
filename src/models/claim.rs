use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use super::claims_identity::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claim {
    /// The issuer of the claim
    issuer: String,

    /// The original issuer of the claim
    original_issuer: String,

    /// Additional properties associated with this claim.
    properties: HashMap<String, String>,

    /// The subject of the claim
    subject: ClaimsIdentity,

    /// The type of the claim
    type_: String,

    /// The value of the claim
    value: String,

    /// The value type of the claim
    value_type: String
}

impl Hash for Claim {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.issuer.hash(state);
        self.original_issuer.hash(state);

        for (name, value) in &self.properties {
            name.hash(state);
            value.hash(state);
        }

        self.subject.hash(state);
        self.type_.hash(state);
        self.value.hash(state);
        self.value_type.hash(state);
    }
}