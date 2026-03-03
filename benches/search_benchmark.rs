use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn search_benchmark(_c: &mut Criterion) {
    // Placeholder benchmark - will be implemented in Task 7
}

criterion_group!(benches, search_benchmark);
criterion_main!(benches);
