use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use post_benchmarks::siphash::work;

pub fn siphash_benchmark(c: &mut Criterion) {
    let mb = 1024 * 1024;
    let size = 32 * mb;

    let mut group = c.benchmark_group("siphash");
    group.throughput(Throughput::Bytes(size));
    group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
        b.iter(|| work(size as u64));
    });

    group.finish();
}

criterion_group!(benches, siphash_benchmark);
criterion_main!(benches);
