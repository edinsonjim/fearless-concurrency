async fn fetch_data() -> String {
    String::from("iDesoft Systems")
}

async fn process() {
    println!("Starting...");
    // Execution pauses here, but the THREAD is not blocked
    let data: String = fetch_data().await;
    println!("Done with: {}", data);
}

async fn main() {
    let response = process().await;
}
