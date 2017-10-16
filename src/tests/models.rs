mod common;

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        println!(common::example());
        
        assert_eq!(2 + 2, 4);
    }
}