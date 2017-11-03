use std::str::FromStr;
use std::string::ToString;
use std::result::Result;
use std::result::Result::{Ok, Err};

use utils::enum_parse_error::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    AccessToken,
    IdentityToken
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match *self {
            TokenType::AccessToken => String::from("access_token"),
            TokenType::IdentityToken => String::from("id_token")
        }
    }
}

impl FromStr for TokenType {
    type Err = EnumParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token_type = match s {
            "access_token" => TokenType::AccessToken,
            "id_token" => TokenType::IdentityToken,
            "" => return Err(EnumParseError::Empty),
            _ => return Err(EnumParseError::NoMatch(s.to_owned()))
        };

        Ok(token_type)
    }
}