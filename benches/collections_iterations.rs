use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};

const COLLECTION_SIZE: usize = 1_000_000;

#[derive(Copy, Clone)]
struct CollectionValue {
    inc: u32
}

impl CollectionValue {
    fn new() -> Self {
        CollectionValue {
            inc: 1
        }
    }

    fn set_inc(&mut self, inc: u32) {
        self.inc = inc;
    }
}

fn iteration_array(a: &[CollectionValue; COLLECTION_SIZE], delta: usize) -> u32 {
    let mut count = delta as u32;
    for value in a.iter() {
        count = value.inc;
    }
    count
}

fn iterate_vector(v: &Vec<CollectionValue>, delta: usize) -> u32 {
    let mut count = delta as u32;
    for value in v.iter() {
        count = value.inc;
    }
    count
}

fn bench_collections_preallocated(c: &mut Criterion) {
    let mut group = c.benchmark_group("CollectionPreallocated");
    let parameter = format!("[CollectionValue; {}]", COLLECTION_SIZE);

    {
        let delta = black_box(0);
        let mut array = [CollectionValue::new(); COLLECTION_SIZE];
        for i in 0..COLLECTION_SIZE {
            let v = i as u32;
            array[i].set_inc(v);
        }

        group.bench_with_input(
            BenchmarkId::new("Array", &parameter),
            &array,
            |b, i| b.iter(|| iteration_array(i, delta))
        );
    }

    {
        let delta = black_box(0);
        let mut vector = vec![CollectionValue::new(); COLLECTION_SIZE];
        for i in 0..COLLECTION_SIZE {
            let v = i as u32;
            vector[i].set_inc(v);
        }

        group.bench_with_input(
            BenchmarkId::new("Vector", &parameter),
            &vector,
            |b, i| b.iter(|| iterate_vector(i, delta))
        );
    }

    group.finish();
}

criterion_group!(benches, bench_collections_preallocated);
criterion_main!(benches);