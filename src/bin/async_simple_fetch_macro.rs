use std::time::Duration;

async fn fetch_data() -> String {
    println!("fetching data...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    String::from("iDesoft Systems")
}

async fn do_something() {
    println!("do something");
    tokio::time::sleep(Duration::from_secs(2)).await;

    println!("hello");
}

#[tokio::main]
async fn main() {
    do_something().await;
    let company = fetch_data().await;
    println!("company name: {}", company);
}
