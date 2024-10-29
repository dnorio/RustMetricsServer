use criterion::{criterion_group, criterion_main, Criterion};
use reqwest::Client;
use tokio::runtime::Runtime;

async fn make_request(client: &Client) -> reqwest::Result<()> {
    let response = client.get("http://localhost:3000").send().await?;

    assert!(response.status().is_success());
    Ok(())
}

fn benchmark_requests(c: &mut Criterion) {
    let client = Client::new();
    let rt = Runtime::new().unwrap();

    c.bench_function("benchmark_requests", |b| {
        b.iter(|| {
            rt.block_on(make_request(&client)).unwrap();
        });
    });
}

criterion_group!(benches, benchmark_requests);
criterion_main!(benches);
