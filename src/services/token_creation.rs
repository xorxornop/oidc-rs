trait TokenCreationService {
    fn get_profile_data(context: &ProfileDataRequestContext) -> ProfileDataRequestContext {
        unimplemented!();
    }
}

#[derive(Debug)]
struct DefaultTokenCreationService {
    field: u32
}

impl TokenCreationService for DefaultTokenCreationService {
    // add code here
}
