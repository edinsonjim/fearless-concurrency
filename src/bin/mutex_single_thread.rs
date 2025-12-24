use std::{sync::Mutex, thread};

fn main() {
    let treasure_chest: Mutex<i32> = Mutex::new(0);

    let handle = thread::spawn(move || {
        let mut data = treasure_chest.lock().unwrap();
        *data += 100;
        println!("Value inside the lock: {}", *data);
    });

    handle.join().ok();
    // let final_data = treasure_chest.lock().unwrap(); // value borrowed here after move
    // println!("Final value: {}", *final_data);
}
