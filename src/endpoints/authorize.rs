use std::collections::HashMap;

type AuthorizeResponse = HashMap<String, String>;

#[post("/authorize")]
fn post_authorize() -> Json<AuthorizeResponse> {
    let response = generate_response();

    response
}

/// Generate token response
fn generate_response(user_id: &str) -> AuthorizeResponse {
    let response = TokenResponse::new();
    
    response
}



