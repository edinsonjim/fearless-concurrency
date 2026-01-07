use std::time::Duration;

#[tokio::main]
async fn main() {
    let (tx, _) = tokio::sync::watch::channel("v1");

    // subscriptor 1
    // let mut rx1 = tx.subscribe();
    // tokio::spawn(async move {
    //     println!("[subscriber 1] subscribed");
    //     while rx1.changed().await.is_ok() {
    //         let value = *rx1.borrow();
    //         println!("[subscriber 1] detected: {}", value);
    //     }
    // });

    // subscriptor 2
    let mut rx2 = tx.subscribe();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        println!("[subscriber 2] subscribed");
        while rx2.changed().await.is_ok() {
            let value = *rx2.borrow();
            println!("[subscriber 2] detected: {}", value);
        }
    });

    tokio::spawn(async move {
        tx.send("v2").unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
        tx.send("v3").unwrap();
        tokio::time::sleep(Duration::from_secs(1)).await;
        tx.send("v4").unwrap();
    });

    tokio::time::sleep(Duration::from_secs(5)).await;
}
