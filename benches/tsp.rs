use criterion::{criterion_group, criterion_main, Criterion};
use rand::{Rng, SeedableRng};
use rand_chacha;
use tsp_rust::{tsp, Town};

pub fn tsp_benchmark(c: &mut Criterion) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    let towns: Vec<Town> = (0..10)
        .map(|_| Town {
            x: rng.gen(),
            y: rng.gen(),
        })
        .collect();
    c.bench_function("10 towns", |b| b.iter(|| tsp::tsp_solve(&towns)));

    let towns: Vec<Town> = (0..11)
        .map(|_| Town {
            x: rng.gen(),
            y: rng.gen(),
        })
        .collect();
    c.bench_function("11 towns", |b| b.iter(|| tsp::tsp_solve(&towns)));

    let towns: Vec<Town> = (0..12)
        .map(|_| Town {
            x: rng.gen(),
            y: rng.gen(),
        })
        .collect();
    c.bench_function("12 towns", |b| b.iter(|| tsp::tsp_solve(&towns)));
}

criterion_group!(benches, tsp_benchmark);
criterion_main!(benches);
