use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use wasm_game_of_life::universe::Universe;

fn universe_step_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Universe");

    for universe_size in [1, 10, 100, 1000] {
        let universe = Universe::new_random(universe_size, universe_size);

        group.bench_function(BenchmarkId::new("step", universe_size), |b| {
            b.iter(|| universe.step())
        });
    }
}

criterion_group!(universe_benches, universe_step_benchmark);
criterion_main!(universe_benches);
