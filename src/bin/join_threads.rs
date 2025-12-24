use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

pub fn main() {
    println!("[main] started");
    // spawn a new thread
    let handle: JoinHandle<()> = thread::spawn(|| {
        for i in 1..=5 {
            println!("[child] number {}!", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    println!("[main] doing other work");

    // Block until the thread finishes and get its result
    let result = handle.join();
    println!("[main] child finished with result: {:?}", result);
    println!("[main] finished");
}
