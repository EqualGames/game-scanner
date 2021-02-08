use criterion::{criterion_group, criterion_main, Criterion};

fn list(c: &mut Criterion) {
    c.bench_function("games", |b| {
        b.iter(|| {
            let games = gamescanner::games();
            assert_ne!(games.len(), 0);
        })
    });
}

criterion_group!(benches, list);
criterion_main!(benches);
