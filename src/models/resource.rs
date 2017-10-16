use std::vec::Vec;

pub trait Resource {
    fn new(name: String) -> Self;
    fn new_with_claims(name: String, user_claims: Vec<String>) -> Self;

    fn enabled(&self) -> bool;
    fn name(&self) -> &String;
    fn display_name(&self) -> &String;
    fn description(&self) -> &String;
    fn user_claims(&self) -> &Vec<String>;
}