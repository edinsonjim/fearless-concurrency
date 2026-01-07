struct FetchCompany;

impl Future for FetchCompany {
    type Output = String;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::task::Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    println!("fetching...");
    let fetcher = FetchCompany;
    let company_name = fetcher.await;
    println!("company: {}", company_name);
}
