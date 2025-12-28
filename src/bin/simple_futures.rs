use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

enum State {
    Idle,
    Pending,
    Ready(String),
}

struct FetchCompanyName {
    state: Arc<Mutex<State>>,
}

impl FetchCompanyName {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(State::Idle)),
        }
    }
}

impl Future for FetchCompanyName {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().unwrap();

        match &*state {
            State::Idle => {
                *state = State::Pending;

                let state_clone = Arc::clone(&self.state);
                let waker = cx.waker().clone();

                thread::spawn(move || {
                    thread::sleep(Duration::from_secs(2));
                    let mut state = state_clone.lock().unwrap();

                    *state = State::Ready("iDesoft Systems!".into());

                    waker.wake(); // Notifica al executor que el valor está listo
                });

                Poll::Pending
            }
            State::Pending => Poll::Pending,
            State::Ready(name) => Poll::Ready(name.clone()),
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Fetching...");
    let company = FetchCompanyName::new().await;
    println!("company name: {}", company);
}
