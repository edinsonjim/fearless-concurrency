#[tokio::main]
async fn main() {
    // Capacity of 16 messages
    let (tx, _) = tokio::sync::broadcast::channel(5);
    let mut rx2 = tx.subscribe(); // New subscriber!
    let mut rx3 = tx.subscribe(); // New subscriber!

    tx.send("Hello Everyone!").unwrap();

    println!("rx2: {}", rx2.recv().await.unwrap());
    println!("rx3: {}", rx3.recv().await.unwrap());
}
