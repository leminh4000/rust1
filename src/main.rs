mod print;
mod regex;
mod option;
mod request;
mod my_enum;
mod gcd;

fn main() {
    // print::run();

    // regex::run();
    // option::run();
    // request::run();
    // my_enum::run();
    // gcd::run();
    let mut book= Vec::new();
    book.push(String::from("hello"));
    {
        let ref_book = &book;
        println!("{:?}",ref_book);
        book.push(String::from("world"));
    }
    println!("{:?}",book);
}

/*#[cfg(test)]
mod test2 {
    use crate::gcd;

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

    #[test]
    fn test_gcd() {
        assert_eq!(2, gcd::gcd(2284082094829042, 2293048902839048));
        assert_eq!(gcd::gcd(2 * 3 * 5 * 11 * 17,
                       3 * 7 * 11 * 13 * 19),
                   3 * 11);
    }
}*/



