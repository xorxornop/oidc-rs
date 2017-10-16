use std::option::Option;
use std::iter::*;


/// Models an OpenID Connect or OAuth2 client
#[derive(Debug)]
struct Client {
    field: String
}

impl Client {
    fn validate_grant_types(grant_types: [&str]) {
        unimplemented!();
    }
}