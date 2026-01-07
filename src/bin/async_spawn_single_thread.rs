use std::time::Duration;

use tokio::task;

async fn heavy_work_no_async() {
    println!("heavy work starting");
    std::thread::sleep(Duration::from_secs(3));
    println!("heavy work complete");
}

async fn async_task() {
    println!("async task starting");
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("async task complete")
}

fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        // task::spawn_blocking(heavy_work_no_async);
        // WARNING!!! attention with this blocking the runtime
        task::spawn(heavy_work_no_async());
        task::spawn(async_task());

        tokio::time::sleep(Duration::from_secs(8)).await;
    });
}
