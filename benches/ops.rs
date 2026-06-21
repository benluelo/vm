use std::{hint::black_box, iter};

use criterion::{
    BenchmarkGroup, Criterion, criterion_group, criterion_main, measurement::WallTime,
};
use rand::RngExt;
use vm::{Op, Vm};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ops");

    op_bench(&mut group, "add", Op::ADD);
    op_bench(&mut group, "sub", Op::SUB);
    op_bench(&mut group, "mul", Op::MUL);

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

fn op_bench(group: &mut BenchmarkGroup<'_, WallTime>, name: &str, op: Op) {
    group.bench_function(name, |b| {
        let mut rng = rand::rng();

        b.iter_batched(
            || Vm {
                code: (0..10_000).flat_map(|_| op.to_bytes()).collect(),
                data: vec![],
                stack: (0..10_001).map(|_| rng.random()).collect(),
                memory: vec![],
                cycles: 0,
            },
            |mut vm| black_box(vm.run().unwrap()),
            criterion::BatchSize::SmallInput,
        );
    });
}
