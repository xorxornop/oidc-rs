#[derive(Debug)]
struct ProfileDataRequestContext {
    subject: String,
    requested_claim_types: Vec<String>,
    client: String,
    caller: String,
    issued_claims: Vec<String>,
}

impl ProfileDataRequestContext {
    // add code here
}

trait ProfileService {
    fn get_profile_data(context: &ProfileDataRequestContext) -> ProfileDataRequestContext {
        unimplemented!();
    }
}

#[derive(Debug)]
struct DefaultProfileService {
    field: u32
}

impl ProfileService for DefaultProfileService {
    // add code here
}
