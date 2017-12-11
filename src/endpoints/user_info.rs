use std::collections::HashMap;
use std::vec::Vec;
use std::iter::*;

type UserInfoResponse = HashMap<String, String>;

#[get("/userinfo")]
fn get_user_info() -> Json<UserInfoResponse> {
    let response = generate_response("");

    response
}

/// Get claims for the user
fn generate_response(user_id: &str) -> UserInfoResponse {
    let response = UserInfoResponse::new();
    
    // Populate claims
    
    response
}

fn get_requested_claim_types(user_id: &str, scopes: Vec<String>) -> Vec<String> {
    // Get all identity resources/scopes
    let identity_resources = get_identity_resources(&user_id);

    // Add the claims from each requested scope
    let scope_claims = Vec<String>::new();
    for scope in scopes {
        match identity_resources.first(|ir| ir.name == scope) {
            Some(ir_detail) => scope_claims.add_range(ir_detail.user_claims),
            None => _,
        }
    }

    // Return the distinct claims
    scope_claims.distinct().to_vec()
}

fn get_identity_resources() -> Vec<String> {
    



    unimplemented!();
}