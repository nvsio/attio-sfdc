//! Sync benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sync_benchmark(_c: &mut Criterion) {
    // TODO: Add sync benchmarks
}

criterion_group!(benches, sync_benchmark);
criterion_main!(benches);
