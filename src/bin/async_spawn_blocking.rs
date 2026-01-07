use std::time::Duration;

fn heavy_work_no_async() {
    println!("heavy work starting");
    std::thread::sleep(Duration::from_secs(3));
    println!("heavy work complete");
}
async fn async_task() {
    println!("async task starting");
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("async task complete")
}

#[tokio::main]
async fn main() {
    tokio::task::spawn_blocking(heavy_work_no_async);
    tokio::task::spawn(async_task());
    tokio::time::sleep(Duration::from_secs(4)).await;
}
