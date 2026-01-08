use std::thread;

fn main() {
    // let value = 10;
    let value = String::from("iDesoft");
    // let value = Arc::new(42);

    thread::spawn(move || {
        println!("value: {}", value);
    })
    .join()
    .ok();
}
