use criterion::{criterion_group, criterion_main, Criterion};

fn list(c: &mut Criterion) {
    c.bench_function("amazon::games", |b| {
        b.iter(|| {
            let games = game_scanner::amazon::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
    c.bench_function("blizzard::games", |b| {
        b.iter(|| {
            let games = game_scanner::blizzard::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
    c.bench_function("epicgames::games", |b| {
        b.iter(|| {
            let games = game_scanner::epicgames::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
    c.bench_function("gog::games", |b| {
        b.iter(|| {
            let games = game_scanner::gog::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
    c.bench_function("origin::games", |b| {
        b.iter(|| {
            let games = game_scanner::origin::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
    c.bench_function("riotgames::games", |b| {
        b.iter(|| {
            let games = game_scanner::riotgames::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
    c.bench_function("steam::games", |b| {
        b.iter(|| {
            let games = game_scanner::steam::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
    c.bench_function("ubisoft::games", |b| {
        b.iter(|| {
            let games = game_scanner::ubisoft::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });
}

criterion_group!(benches, list);
criterion_main!(benches);
