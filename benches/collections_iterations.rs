use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};

const COLLECTION_SIZE: usize = 1_000;

#[derive(Copy, Clone)]
struct CollectionValue {
    inc: u16
}

impl CollectionValue {
    fn new() -> Self {
        CollectionValue {
            inc: 1
        }
    }
}

fn iteration_array(a: &[CollectionValue; COLLECTION_SIZE]) -> u16 {
    let mut count = 0;
    for value in a.iter() {
        count += value.inc;
    }
    count
}

fn iterate_vector(v: &Vec<CollectionValue>) -> u16 {
    let mut count = 0;
    for value in v.iter() {
        count += value.inc;
    }
    count
}

fn bench_collections_preallocated(c: &mut Criterion) {
    let mut group = c.benchmark_group("CollectionPreallocated");

    {
        let array = black_box([CollectionValue::new(); COLLECTION_SIZE]);
        group.bench_with_input(
            BenchmarkId::new("Array", "[CollectionValue; n]"),
            &array,
            |b, i| b.iter(|| iteration_array(i))
        );
    }

    {
        let vector = black_box(vec![CollectionValue::new(); COLLECTION_SIZE]);
        group.bench_with_input(
            BenchmarkId::new("Vector", "[CollectionValue; n]"),
            &vector,
            |b, i| b.iter(|| iterate_vector(i))
        );
    }

    group.finish();
}

criterion_group!(benches, bench_collections_preallocated);
criterion_main!(benches);