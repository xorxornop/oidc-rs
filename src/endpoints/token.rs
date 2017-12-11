use std::collections::HashMap;

type TokenResponse = HashMap<String, String>;

#[get("/token")]
fn get_token() -> Json<TokenResponse> {
    let response = generate_response();

    response
}

/// Generate token response
fn generate_response(user_id: &str) -> TokenResponse {
    let token = TokenResponse::new();
    
    token
}