use criterion::{criterion_group, criterion_main, Criterion};

fn test_bench(c: &mut Criterion) {
    c.bench_function("test benchmark", |b| b.iter(|| {}));
}

criterion_group!(bench, test_bench);
criterion_main!(bench);
