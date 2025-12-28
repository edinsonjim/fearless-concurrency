use std::{
    sync::{Arc, Mutex},
    thread,
};

struct Person {
    id: i32,
    username: String,
}

struct SharedState {
    people: Vec<Person>,
}

impl SharedState {
    pub fn new() -> Self {
        Self { people: Vec::new() }
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.push(person);
    }
}

fn main() {
    let state = SharedState::new();
    let shared_space = Arc::new(Mutex::new(state));

    let mut handles = vec![];

    for thread_index in 0..10 {
        let shared_ref = Arc::clone(&shared_space);
        let handle = thread::spawn(move || {
            let mut state = shared_ref.lock().unwrap();
            let username = format!("idesoft-{}", thread_index);

            state.add_person(Person {
                id: thread_index,
                username,
            });
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_state = shared_space.lock().unwrap();
    println!("Final state: {} people added", final_state.people.len());
    for person in &final_state.people {
        println!("  person id: {}, username: {}", person.id, person.username);
    }
}
