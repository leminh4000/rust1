pub fn run(){
    println!("{:?}", get_occupation("Le Van Minh2").unwrap_or("Unknown"));
}
fn get_occupation(name: &str) -> Option<&str>{
    match name{
        "Le Van Minh" => Some("Student"),
        "Nguyen Van A" => Some("Teacher"),
        _ => None,
    }
}