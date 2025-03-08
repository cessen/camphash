use camphash::hash256;
use criterion::{criterion_group, criterion_main, Criterion, Throughput};

//----

fn hash256_throughput(c: &mut Criterion) {
    let benches = [
        // ("10b_message", 10),       // 10-byte input.
        // ("100b_message", 100),     // 100-byte input.
        // ("1kb_message", 1000),     // 1-kilobyte input.
        // ("10kb_message", 10000),   // 10-kilobyte input.
        // ("100kb_message", 100000), // 100-kilobyte input.
        ("1mb_message", 1000000), // 1-megabyte input.
    ];

    let mut group = c.benchmark_group("hash256");

    for (name, data_size) in benches.iter() {
        let data: Vec<u8> = b"abcdefghijklmnopqrstuvwxyz"
            .iter()
            .copied()
            .cycle()
            .take(*data_size)
            .collect();
        group.throughput(Throughput::Bytes(*data_size as u64));

        group.bench_function(*name, |bench| {
            bench.iter(|| {
                let _ = hash256(&data);
            })
        });
    }
}

//----

criterion_group!(benches, hash256_throughput);
criterion_main!(benches);
