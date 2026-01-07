use std::sync::Arc;

#[tokio::main]
async fn main() {
    let counter = Arc::new(tokio::sync::Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..100 {
        let counter_ref = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            let mut counter_data = counter_ref.lock().await;
            *counter_data += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Final count: {}", *counter.lock().await);
}
