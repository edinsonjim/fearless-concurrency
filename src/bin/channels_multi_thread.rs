use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

fn main() {
    // Create a channel
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // Clone the transmitter for each thread
    let tx_for_thread_1 = tx.clone();
    let tx_for_thread_2 = tx.clone();

    // Spawn thread and move transmitter (tx) into it
    thread::spawn(move || {
        let val = String::from("hi from thread 1");
        tx_for_thread_1.send(val).unwrap();
        // val is moved. We can't use it here anymore.
    });

    thread::spawn(move || {
        let val = String::from("hi from thread 2");
        tx_for_thread_2.send(val).unwrap();
        // val is moved. We can't use it here anymore.
    });

    tx.send("Hello".into()).unwrap();

    // 1. Drop the original transmitter so the receiver knows when to stop
    // drop(tx);

    // 2. move inside another scope with {}
    {
        tx
    };

    // The main thread (consumer) iterates over received messages
    // The for received in rx loop only stops when the channel is closed.
    for received in rx {
        println!("received: {}", received);
    }
    println!("Finished");
}
