use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

const MB: u64 = 1024 * 1024;

pub fn benchmark_blake(c: &mut Criterion) {
    let size = 16 * MB;

    let mut group = c.benchmark_group("blake3");
    group.throughput(Throughput::Bytes(size));
    group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
        b.iter(|| post_benchmarks::blake3::work(black_box(size as u64)));
    });

    group.finish();
}

pub fn benchmark_aes(c: &mut Criterion) {
    let size = 128 * MB;

    let mut group = c.benchmark_group("aes");
    group.throughput(Throughput::Bytes(size));
    group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
        b.iter(|| black_box(post_benchmarks::aes::work_many(4, black_box(size as u64))));
    });

    group.finish();
}

// criterion_group!(
//     name = benches;
//     config = Criterion::default().with_profiler(PProfProfiler::new(1000, Output::Flamegraph(None)));
//     targets = blake3_benchmark
// );

criterion_group!(benches, benchmark_aes);
criterion_main!(benches);
