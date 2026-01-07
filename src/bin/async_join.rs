use std::time::Duration;

async fn task_one() -> String {
    tokio::time::sleep(Duration::from_millis(2)).await;
    String::from("iDesoft")
}

async fn task_two() -> String {
    tokio::time::sleep(Duration::from_millis(2)).await;
    String::from("Systems")
}

#[tokio::main]
async fn main() {
    let (res1, res2) = tokio::join!(task_one(), task_two());
    println!("res1 {} - res2: {}", res1, res2);
}
