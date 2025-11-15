use std::{hint::black_box, iter};

use native_histogram::{
    find_bucket, go_find_bucket, powi, schema::BoundedSchema, search, ZERO_THRESHOLD,
};

const SCHEMAS: &[isize] = &[-4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8];

fn bench<S: Copy>(schema: S, find_bucket: impl Fn(S, f64, f64) -> Option<(i32, bool)>) {
    static VALUES: std::sync::LazyLock<Vec<f64>> =
        std::sync::LazyLock::new(|| iter::repeat_with(fastrand::f64).take(100).collect());
    for &value in &*VALUES {
        black_box(find_bucket(
            black_box(schema),
            black_box(ZERO_THRESHOLD),
            value,
        ));
    }
}

#[divan::bench(types = [powi::Naive, powi::Loop, powi::Match, powi::BoundedLoop, powi::UnrolledLoop, powi::NestedLoops], consts = [false, true], args = SCHEMAS)]
fn rust<Powi: powi::Strategy, const BOUNDED_SCHEMA: bool>(schema: isize) {
    if BOUNDED_SCHEMA {
        let schema = BoundedSchema::try_from(schema).unwrap();
        bench(schema, find_bucket::<Powi>);
    } else {
        bench(schema, find_bucket::<Powi>);
    }
}

#[divan::bench(types = [search::Binary, search::Linear, search::Hybrid, search::BinaryInlined], consts = [false, true], args = SCHEMAS)]
fn go<Search: search::Strategy, const BOUNDED_SCHEMA: bool>(schema: isize) {
    bench(schema, go_find_bucket::<Search>);
}

fn main() {
    divan::main()
}
