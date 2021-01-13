use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box};
use crossbeam::queue::ArrayQueue;

const COLLECTION_SIZE: usize = 1000;

fn array_write_isize(a: &mut [isize; COLLECTION_SIZE]) -> isize {
    for i in 0..a.len()-1 {
        a[i] = i as isize;
    }
    a[a.len() - 1]
}

fn crossbeam_array_write_isize(a: &mut ArrayQueue<isize>) -> isize {
    for i in 0..a.capacity()-1 {
        let _ = a.push(i as isize);
    }
    a.pop().unwrap()
}

fn bench_write(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArrayWrite");
    let parameter = format!("[isize; {}]", COLLECTION_SIZE);

    {
        let mut array: [isize; COLLECTION_SIZE] = [0; COLLECTION_SIZE];

        group.bench_function(
            BenchmarkId::new("Array", &parameter),
            move |b| {
                b.iter(|| array_write_isize(&mut array))
            }
        );
    }

    {
        let mut array: ArrayQueue<isize> = ArrayQueue::new(COLLECTION_SIZE);

        group.bench_function(
            BenchmarkId::new("CrossbeamArray", &parameter),
            move |b| {
                b.iter(|| crossbeam_array_write_isize(&mut array))
            }
        );
    }

    group.finish();
}

criterion_group!(benches, bench_write);
criterion_main!(benches);