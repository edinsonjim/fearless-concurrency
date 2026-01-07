#[tokio::main]
async fn main() {
    // Create a channel with a capacity of 32 messages
    let (tx, mut rx) = tokio::sync::mpsc::channel(5);

    for i in 0..5 {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            let _ = tx_clone.send(format!("Task {} reports: Success!", i)).await;
        });
    }

    // Drop the original sender so the receiver knows when to stop
    drop(tx);

    while let Some(message) = rx.recv().await {
        println!("Manager received: {}", message);
    }
}
