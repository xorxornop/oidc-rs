#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClaimsIdentity {
    authentication_type: String,
    name: String,
    label: String
}