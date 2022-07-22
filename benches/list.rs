use criterion::*;

fn list(c: &mut Criterion) {
    let mut group = c.benchmark_group("list");

    #[cfg(target_os = "windows")]
    group.bench_function("amazon::games", |b| {
        b.iter(|| {
            let games = game_scanner::amazon::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });

    #[cfg(not(target_os = "linux"))]
    group.bench_function("blizzard::games", |b| {
        b.iter(|| {
            let games = game_scanner::blizzard::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });

    #[cfg(not(target_os = "linux"))]
    group.bench_function("epicgames::games", |b| {
        b.iter(|| {
            let games = game_scanner::epicgames::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });

    #[cfg(not(target_os = "linux"))]
    group.bench_function("gog::games", |b| {
        b.iter(|| {
            let games = game_scanner::gog::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });

    #[cfg(not(target_os = "linux"))]
    group.bench_function("origin::games", |b| {
        b.iter(|| {
            let games = game_scanner::origin::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });

    #[cfg(not(target_os = "linux"))]
    group.bench_function("riotgames::games", |b| {
        b.iter(|| {
            let games = game_scanner::riotgames::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });

    #[cfg(not(target_os = "linux"))]
    group.bench_function("steam::games", |b| {
        b.iter(|| {
            let games = game_scanner::steam::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });

    #[cfg(target_os = "windows")]
    group.bench_function("ubisoft::games", |b| {
        b.iter(|| {
            let games = game_scanner::ubisoft::games().unwrap();
            assert_ne!(games.len(), 0);
        })
    });

    group.finish();
}

criterion_group!(benches, list);
criterion_main!(benches);
