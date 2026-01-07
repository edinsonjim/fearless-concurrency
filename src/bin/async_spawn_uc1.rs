use std::time::Duration;

async fn task(task_id: i32) -> String {
    println!("starting task {}...", task_id);
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("task {} finished", task_id);
    format!("done {}", task_id)
}

#[tokio::main]
async fn main() {
    let task1_handle = tokio::spawn(task(2));
    let task2_handle = tokio::spawn(task(3));

    let task1_val = task1_handle.await.unwrap();
    let task2_val = task2_handle.await.unwrap();

    println!("task1 value: {}", task1_val);
    println!("task2 value: {}", task2_val);

    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("finished");
}
