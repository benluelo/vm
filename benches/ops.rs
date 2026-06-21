use std::{hint::black_box, iter};

use criterion::{Criterion, criterion_group, criterion_main};
use rand::RngExt;
use vm::{Op, Vm};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ops");

    group.bench_function("add", |b| {
        let mut rng = rand::rng();

        b.iter_batched(
            || Vm {
                code: (0..10_000).flat_map(|_| Op::ADD.to_bytes()).collect(),
                data: vec![],
                stack: (0..10_001).map(|_| rng.random()).collect(),
                memory: vec![],
                cycles: 0,
            },
            |mut vm| black_box(vm.run().unwrap()),
            criterion::BatchSize::SmallInput,
        )
    });

    group.bench_function("mul", |b| {
        let mut rng = rand::rng();

        b.iter_batched(
            || Vm {
                code: (0..10_000).flat_map(|_| Op::MUL.to_bytes()).collect(),
                data: vec![],
                stack: (0..10_001).map(|_| rng.random()).collect(),
                memory: vec![],
                cycles: 0,
            },
            |mut vm| black_box(vm.run().unwrap()),
            criterion::BatchSize::SmallInput,
        );
    });

    group.bench_function("div", |b| {
        b.iter_batched(
            || Vm {
                code: (0..10_000).flat_map(|_| Op::DIV.to_bytes()).collect(),
                data: vec![],
                stack: (0..10_001)
                    // .map(|_| rng.random_range(1..u64::MAX))
                    .map(|_| 1)
                    .collect(),
                memory: vec![],
                cycles: 0,
            },
            |mut vm| black_box(vm.run().unwrap()),
            criterion::BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
