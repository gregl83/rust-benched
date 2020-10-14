use criterion::{criterion_group, criterion_main, black_box, Criterion};

pub fn exponent(a: i32, b: i32) -> i32 {
    a ^ b
}

pub fn bench_exponents(c: &mut Criterion) {
    c.bench_function("exponent function", |b| b.iter(|| {
        let n = black_box(1000);
        (0..n).fold(0, exponent)
    }));
}

criterion_group!(benches, bench_exponents);
criterion_main!(benches);