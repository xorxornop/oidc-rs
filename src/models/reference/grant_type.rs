use std::str::FromStr;
use std::string::ToString;
use std::result::Result;
use std::result::Result::{Ok, Err};

use utils::enum_parse_error::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrantType {
    Implicit,
    Hybrid,
    AuthorizationCode,
    ClientCredentials,
    ResourceOwnerPassword
}

impl ToString for GrantType {
    fn to_string(&self) -> String {
        match *self {
            GrantType::Implicit => String::from("implicit"),
            GrantType::Hybrid => String::from("hybrid"),
            GrantType::AuthorizationCode => String::from("authorization_code"),
            GrantType::ClientCredentials => String::from("client_credentials"),
            GrantType::ResourceOwnerPassword => String::from("password")
        }
    }
}

impl FromStr for GrantType {
    type Err = EnumParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grant_type = match s {
            "implicit" => GrantType::Implicit,
            "hybrid" => GrantType::Hybrid,
            "authorization_code" => GrantType::AuthorizationCode,
            "client_credentials" => GrantType::ClientCredentials,
            "password" => GrantType::ResourceOwnerPassword,
            "" => return Err(EnumParseError::Empty),
            _ => return Err(EnumParseError::NoMatch(s.to_owned()))
        };

        Ok(grant_type)
    }
}