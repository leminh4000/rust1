extern crate regex;
use regex::Regex;

pub(crate) fn run() {
    let re=Regex::new(r"\w{5}").unwrap();
    let text="hssssello alksd hallo";
    println!("Match? {}",re.is_match(text));

    match re.captures(text) {
        Some(caps) => println!("Found match {:?}",caps),
        None => println!("No match")
    }
}