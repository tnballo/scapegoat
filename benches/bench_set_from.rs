use std::collections::BTreeSet;
use std::convert::TryInto;

use criterion::{criterion_group, criterion_main, Criterion};
use scapegoat::SGSet;

mod test_data;
use test_data::{RAND_10_000, SEQ_10_000};

// Benches -------------------------------------------------------------------------------------------------------------

fn bench_from_rand(c: &mut Criterion) {
    let rand_10k: [usize; 10_000] = RAND_10_000.keys.clone().try_into().unwrap();

    c.bench_function("sgs_from_10_000_rand", |b| {
        b.iter(|| {
           let _ = SGSet::from(rand_10k);
        })
    });

    c.bench_function("std_from_10_000_rand", |b| {
        b.iter(|| {
           let _ = BTreeSet::from(rand_10k);
        })
    });
}

fn bench_from_seq(c: &mut Criterion) {
    let seq_10k: [usize; 10_000] = SEQ_10_000.keys.clone().try_into().unwrap();

    c.bench_function("sgs_from_10_000_seq", |b| {
        b.iter(|| {
           let _ = SGSet::from(seq_10k);
        })
    });

    c.bench_function("std_from_10_000_seq", |b| {
        b.iter(|| {
           let _ = BTreeSet::from(seq_10k);
        })
    });
}

criterion_group!(benches, bench_from_rand, bench_from_seq);
criterion_main!(benches);