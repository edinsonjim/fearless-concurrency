use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // Protect data with Mutex, wrap Mutex with Arc to share ownership
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // Clone the Arc (cheap, just increases reference count)
        let counter_ref = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Lock mutex to get mutable access to internal data
            let mut num = counter_ref.lock().unwrap();
            *num += 1;
            // Lock releases here automatically
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    // thread::sleep(Duration::from_millis(3));

    // Read final result
    println!("Result: {}", *counter.lock().unwrap());
}
