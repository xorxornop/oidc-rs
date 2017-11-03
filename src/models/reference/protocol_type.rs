use std::str::FromStr;
use std::string::ToString;
use std::result::Result;
use std::result::Result::{Ok, Err};

use utils::enum_parse_error::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolType {
    OpenIdConnect,
    WsFederation,
    Saml2p
}

impl ToString for ProtocolType {
    fn to_string(&self) -> String {
        match *self {
            ProtocolType::OpenIdConnect => String::from("oidc"),
            ProtocolType::WsFederation => String::from("wsfed"),
            ProtocolType::Saml2p => String::from("saml2p")
        }
    }
}

impl FromStr for ProtocolType {
    type Err = EnumParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let protocol_type = match s {
            "oidc" => ProtocolType::OpenIdConnect,
            "wsfed" => ProtocolType::WsFederation,
            "saml2p" => ProtocolType::Saml2p,
            "" => return Err(EnumParseError::Empty),
            _ => return Err(EnumParseError::NoMatch(s.to_owned()))
        };

        Ok(protocol_type)
    }
}