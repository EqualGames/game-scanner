use criterion::{criterion_group, criterion_main, Criterion};

fn list(c: &mut Criterion) {
    c.bench_function("amazon::games", |b| {
        b.iter(|| {
            let games = gamescanner::amazon::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
    c.bench_function("blizzard::games", |b| {
        b.iter(|| {
            let games = gamescanner::blizzard::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
    c.bench_function("epicgames::games", |b| {
        b.iter(|| {
            let games = gamescanner::epicgames::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
    c.bench_function("gog::games", |b| {
        b.iter(|| {
            let games = gamescanner::gog::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
    c.bench_function("origin::games", |b| {
        b.iter(|| {
            let games = gamescanner::origin::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
    c.bench_function("riotgames::games", |b| {
        b.iter(|| {
            let games = gamescanner::riotgames::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
    c.bench_function("steam::games", |b| {
        b.iter(|| {
            let games = gamescanner::steam::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
    c.bench_function("ubisoft::games", |b| {
        b.iter(|| {
            let games = gamescanner::ubisoft::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
}

criterion_group!(benches, list);
criterion_main!(benches);
