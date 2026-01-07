#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::oneshot::channel();

    tokio::spawn(async move {
        let _ = tx.send("Calculation Result");
    });

    let res = rx.await.unwrap();
    println!("Got: {}", res);
}
