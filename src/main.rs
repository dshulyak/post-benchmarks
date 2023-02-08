use std::time;

fn main() {
    let start = time::SystemTime::now();
    let size = 16 * 1024 * 1024 * 1024;
    post_benchmarks::aes::work_parallel(size);

    let took = start.elapsed().unwrap();
    println!(
        "{} GiB/s",
        ((size as f64) / 1024.0 / 1024.0 / 1024.0) / took.as_secs_f64()
    );
}
