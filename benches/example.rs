#![feature(test)]

extern crate test;

// custom function
pub fn exponent(a: i32, b: i32) -> i32 {
    a ^ b
}

#[cfg(test)]
mod example {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_exponent(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(1000);

            (0..n).fold(0, exponent)
        })
    }
}