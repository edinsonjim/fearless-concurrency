use std::time::Duration;

use tokio::time::sleep;

async fn handle_request(req_id: i32) {
    println!("starting task {}...", req_id);
    sleep(Duration::from_secs(1)).await;
    println!("Task {} finished", req_id);
}

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // block_on: The entry point
    rt.block_on(async {
        // tokio::spawn concurrency
        // these run at the same time on the worker threads
        tokio::spawn(handle_request(1));
        tokio::spawn(handle_request(2));

        // spawn_blocking: offloading
        // this runs on a separate pool so it doesnt' slow down tasks 1 and 2
        let cpu_task = tokio::task::spawn_blocking(|| {
            // imagine a heavy calculation here
            println!("Heavy computation running...");
            std::thread::sleep(Duration::from_secs(2));
            "Done!"
        });
        let result = cpu_task.await.unwrap();
        println!("Background work: {}", result);
    });
}
