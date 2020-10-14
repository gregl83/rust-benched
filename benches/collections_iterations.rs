use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};

const COLLECTION_SIZE: usize = 1_000_000_000;

#[derive(Copy, Clone)]
struct CollectionValue { }

fn iteration_array(a: &[CollectionValue; COLLECTION_SIZE]) -> i32 {
    let mut count = 0;
    for _ in a.iter() {
        count += 1;
    }
    count
}

fn iterate_vector(v: &Vec<CollectionValue>) -> i32 {
    let mut count = 0;
    for _ in v.iter() {
        count += 1;
    }
    count
}

fn bench_collections_preallocated(c: &mut Criterion) {
    let mut group = c.benchmark_group("CollectionPreallocated");

    let array = black_box([CollectionValue{}; COLLECTION_SIZE]);
    group.bench_with_input(
        BenchmarkId::new("Array", "[CollectionValue; n]"),
        &array,
        |b, i| b.iter(|| iteration_array(i))
    );

    let vector = black_box(vec![CollectionValue{}; COLLECTION_SIZE]);
    group.bench_with_input(
        BenchmarkId::new("Vector", "[CollectionValue; n]"),
        &vector,
        |b, i| b.iter(|| iterate_vector(i))
    );
    group.finish();
}

criterion_group!(benches, bench_collections_preallocated);
criterion_main!(benches);