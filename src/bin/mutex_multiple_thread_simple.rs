use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let counter_ref: Arc<Mutex<i32>> = Arc::clone(&counter);
    thread::spawn(move || {
        // Lock mutex to get mutable access to internal data
        let mut num = counter_ref.lock().unwrap();
        *num += 1;
        // Lock releases here automatically
    });

    let counter_ref: Arc<Mutex<i32>> = Arc::clone(&counter);
    thread::spawn(move || {
        // Lock mutex to get mutable access to internal data
        let mut num = counter_ref.lock().unwrap();
        *num += 1;
        // Lock releases here automatically
    });

    thread::sleep(Duration::from_millis(3));
    let final_data = counter.lock().unwrap();
    println!("Final value: {}", *final_data);
}
