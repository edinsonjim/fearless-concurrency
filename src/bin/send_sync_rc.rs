use std::{cell::RefCell, rc::Rc, sync::Arc, thread};

fn do_something(rc: Rc<i32>) {
    println!("value: {}", rc);
}

fn main() {
    let value = Rc::new(42);
    // do_something(value);

    thread::spawn(move || {
        // TODO(ej): we cannot transfer the ownership between threads
        // just we can transfer the ownership in a single thread.
        // println!("value: {:?}", value);
    })
    .join()
    .ok();
}
