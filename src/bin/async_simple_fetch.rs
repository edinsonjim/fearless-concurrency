use std::time::Duration;

async fn fetch_data() -> String {
    println!("fetching data...");

    tokio::time::sleep(Duration::from_secs(2)).await;

    String::from("iDesoft Systems")
}

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        // .enable_io()
        // .enable_time()
        .enable_all()
        .build()
        .unwrap();

    let company = runtime.block_on(fetch_data());
    println!("company name: {}", company);
}
