use std::{
    task::Poll,
    time::{Duration, Instant},
};

struct SimpleTimer {
    expiration: Instant,
}

impl Future for SimpleTimer {
    type Output = String;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        println!("[SimpleTimer] poll invoked");
        if Instant::now() >= self.expiration {
            println!("[SimpleTimer] poll is ready");
            return Poll::Ready("Timer finished".into());
        }

        println!("[SimpleTimer] preparing waker");
        let waker = cx.waker().to_owned();
        let when = self.expiration;

        // simulate a background thread notifying the waker
        std::thread::spawn(move || {
            println!("[SimpleTimer] [thread] processing in a background thread");
            let now = Instant::now();
            if now < when {
                println!("[SimpleTimer] [thread] keep calm and drink a coffe!!");
                std::thread::sleep(when - now);
            }

            println!("[SimpleTimer] [thread] wake!!!");
            waker.wake(); // tell the executor to poll again
        });

        println!("[SimpleTimer] poll is pending");
        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    println!("[main] started");
    let my_timer = SimpleTimer {
        expiration: Instant::now() + Duration::from_secs(2),
    };

    println!("waiting for the timer...");
    let result = my_timer.await; // The executor calls poll() here
    println!("timer result: {}", result);

    println!("[main] ended");
}

// fn main() {
//     println!("[main] started");
//     task::block_on(async {
//         println!("[task] started");
//         let my_timer = SimpleTimer {
//             expiration: Instant::now() + Duration::from_secs(2),
//         };
//         println!("waiting for the timer (using async-std)...");
//         let result = my_timer.await;
//         println!("[task] result: {}", result);

//         println!("[task] ended");
//     });

//     println!("[main] ended");
// }
