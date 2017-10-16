use chrono::{DateTime, Utc};

/// Models a client secret with identifier and expiration
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Secret {
    /// The description
    description: String,

    /// The value
    value: String,

    /// The expiration time
    expiration: DateTime<Utc>,

    /// The type of the client secret
    type_of: SecretType
}

impl Secret {
    pub fn new(value: String, expiration: DateTime<Utc>) -> Secret {
        Secret::new_with_description(value, "".to_owned(), expiration)
    }

    pub fn new_with_description(value: String, description: String, expiration: DateTime<Utc>) -> Secret {
        Secret {
            value,
            description,
            expiration,
            type_of: SecretType::SharedSecret
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SecretType {
    SharedSecret,
    X509Thumbprint,
    X509CertificateName,
    X509CertificateBase64
}