use conway::game::State;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn state_next_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("state_next");

    let mut inputs = vec![];
    let mut chance = 0.;
    for _ in 0..10 {
        inputs.push(State::random(chance, 100, 100));
        chance += 0.1;
    }

    for i in 0..inputs.len() {
        group.bench_with_input(BenchmarkId::from_parameter(i), &inputs[i], |b, state| {
            b.iter(|| state.next())
        });
    }
}

criterion_group!(benches, state_next_bench);
criterion_main!(benches);
