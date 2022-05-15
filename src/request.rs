extern crate reqwest;

pub async fn run(){
    println!("Requesting.............................");
    match reqwest::get("http://youtube.local/hello").await {
        Ok(mut response) => {
            println!("Response status: {}", response.status());
        },
        Err(e) => println!("Error: {}", e)
    }
}