use std::{thread, time::Duration};

pub fn main() {
    println!("[main] started");
    // spawn a new thread
    thread::spawn(|| {
        for i in 1..5 {
            println!("[child] number {}!", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // main thread
    // for i in 1..3 {
    //     println!("[main] number {}!", i);
    //     thread::sleep(Duration::from_millis(50));
    // }
    // Problem: Main thread might finish before spawned thread!
    // thread::sleep(Duration::from_millis(500));
    println!("[main] finished");
}
