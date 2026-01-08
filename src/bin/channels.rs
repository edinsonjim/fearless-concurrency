use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self};

fn main() {
    // Create a channel
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    // Spawn thread and move transmitter (tx) into it
    thread::spawn(move || {
        let val = String::from("hi from thread");
        tx.send(val).unwrap(); // TODO(ej): if you want to block the receiver comment this
        // val is moved. We can't use it here anymore.
    });

    // Block main thread until a message is received
    let received = rx.recv();
    println!("Got: {:?}", received);
}
