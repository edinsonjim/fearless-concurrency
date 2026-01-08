use std::{cell::RefCell, rc::Rc, sync::Arc, thread};

fn do_something(cell: RefCell<i32>) {
    println!("value: {}", cell.borrow());
}

fn main() {
    // TODO(ej): multiple threads can have access to the same cell.
    // let value = RefCell::new(10);
    // // do_something(value);
    // thread::spawn(move || {
    //     let mut value = value.borrow_mut();
    //     *value = 90;
    //     println!("value: {}", value);
    // })
    // .join()
    // .ok();
    //
    // TODO(ej): we can access because is moved.
    // println!("value cell: {}", value.borrow());

    //
    //
    let value = Arc::new(RefCell::new(10));
    let value_ref = Arc::clone(&value);
    thread::spawn(move || {
        // TODO(ej): we cannot have simultaneous access
        // one thread can read and another thread can modified at the same
        // println!("value: {}", value_ref.borrow());
    });
    println!("value cell: {}", value.borrow());
}
