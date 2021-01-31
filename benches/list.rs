use criterion::{criterion_group, criterion_main, Criterion};

use libgamescanner::*;

fn list_games(c: &mut Criterion) {
    c.bench_function("Amazon", |b| b.iter(|| amazon::games::list()));
    c.bench_function("Epic Games", |b| b.iter(|| epicgames::games::list()));
    c.bench_function("GOG", |b| b.iter(|| gog::games::list()));
    c.bench_function("Origin", |b| b.iter(|| origin::games::list()));
    c.bench_function("Steam", |b| b.iter(|| steam::games::list()));
    c.bench_function("Ubisoft", |b| b.iter(|| ubisoft::games::list()));
}

criterion_group!(benches, list_games);
criterion_main!(benches);
