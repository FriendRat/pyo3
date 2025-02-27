use criterion::{criterion_group, criterion_main, Bencher, Criterion};

use pyo3::prelude::*;
use pyo3::types::PyTuple;

fn iter_tuple(b: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    const LEN: usize = 100_000;
    let tuple = PyTuple::new(py, 0..LEN);
    let mut sum = 0;
    b.iter(|| {
        for x in tuple.iter() {
            let i: u64 = x.extract().unwrap();
            sum += i;
        }
    });
}

fn tuple_get_item(b: &mut Bencher) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    const LEN: usize = 50_000;
    let tuple = PyTuple::new(py, 0..LEN);
    let mut sum = 0;
    b.iter(|| {
        for i in 0..LEN {
            sum += tuple.get_item(i).extract::<usize>().unwrap();
        }
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("iter_tuple", iter_tuple);
    c.bench_function("tuple_get_item", tuple_get_item);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
