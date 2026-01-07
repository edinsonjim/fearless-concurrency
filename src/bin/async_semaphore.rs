use std::sync::Arc;

use tokio::sync::Semaphore;

async fn task(semaphore_ref: Arc<Semaphore>, task_id: i32) {
    let mut _permit = semaphore_ref.acquire().await.unwrap();
    println!("Task {} is working with a permit...", task_id);
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(tokio::sync::Semaphore::new(3)); // Only 3 permits available
    let mut handles = vec![];

    for i in 0..10 {
        let sem_ref = Arc::clone(&semaphore);
        handles.push(tokio::spawn(task(sem_ref, i)));
        // handles.push(tokio::spawn(async move {
        //     let _permit = sem_ref.acquire().await.unwrap();
        //     println!("Task {} is working with a permit...", i);
        //     tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        // }));
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
