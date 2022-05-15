mod print;
mod regex;
mod option;
mod request;
mod my_enum;

fn main() {
    // print::run();

    // regex::run();
    // option::run();
    // request::run();
    // my_enum::run();
}

#[cfg(test)]
mod test2 {
    #[test]
    #[should_panic]
    fn test_basic() {
        assert!(1 == 1);
        panic!("Oh no");
    }

    #[test]
    #[ignore]
    fn test_equals() {
        assert_eq!(2, 1 * 1 + 1);
        assert_ne!(2, 1 * 1 + 1);
    }
}



