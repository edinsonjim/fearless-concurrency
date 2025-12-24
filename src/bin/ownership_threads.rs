use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("vector: {:?}", data);
    });

    let _ = handle.join().ok();
    // println!("vector: {:?}", data);
}
