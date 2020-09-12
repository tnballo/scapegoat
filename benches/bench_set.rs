use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

use scapegoat::SGSet;

// TODO: repetitive code! Macros?

// Rand Test Set Init --------------------------------------------------------------------------------------------------

struct RandTestData {
    keys: Vec<usize>,
    get_idxs: Vec<usize>,
    remove_idxs: Vec<usize>,
}

impl RandTestData {
    fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();

        RandTestData {
            keys: (0..size).map(|_| rng.gen()).collect(),
            get_idxs: (0..size).map(|_| rng.gen_range(0, size)).collect(),
            remove_idxs: (0..size).map(|_| rng.gen_range(0, size)).collect(),
        }
    }
}

// Benches -------------------------------------------------------------------------------------------------------------

fn insert_bench(c: &mut Criterion) {
    let rtd_100 = RandTestData::new(100);
    let rtd_1_000 = RandTestData::new(1_000);
    let rtd_10_000 = RandTestData::new(10_000);

    c.bench_function("sgs_insert_100", |b| {
        b.iter(|| {
            let mut sgs = SGSet::new();
            for k in &rtd_100.keys {
                sgs.insert(*k);
            }
        })
    });

    c.bench_function("sgs_insert_1_000", |b| {
        b.iter(|| {
            let mut sgs = SGSet::new();
            for k in &rtd_1_000.keys {
                sgs.insert(*k);
            }
        })
    });

    c.bench_function("sgs_insert_10_000", |b| {
        b.iter(|| {
            let mut sgs = SGSet::new();
            for k in &rtd_10_000.keys {
                sgs.insert(*k);
            }
        })
    });
}

fn get_bench(c: &mut Criterion) {
    let rtd_100 = RandTestData::new(100);
    let rtd_1_000 = RandTestData::new(1_000);
    let rtd_10_000 = RandTestData::new(10_000);

    let mut sgs_100 = SGSet::new();
    for k in &rtd_100.keys {
        sgs_100.insert(*k);
    }

    let mut sgs_1_000 = SGSet::new();
    for k in &rtd_100.keys {
        sgs_1_000.insert(*k);
    }

    let mut sgs_10_000 = SGSet::new();
    for k in &rtd_10_000.keys {
        sgs_10_000.insert(*k);
    }

    c.bench_function("sgs_get_100", |b| {
        b.iter(|| {
            for k in &rtd_100.remove_idxs {
                sgs_100.get(k);
            }
        })
    });

    c.bench_function("sgs_get_1_000", |b| {
        b.iter(|| {
            for k in &rtd_1_000.remove_idxs {
                sgs_1_000.get(k);
            }
        })
    });

    c.bench_function("sgs_get_10_000", |b| {
        b.iter(|| {
            for k in &rtd_10_000.remove_idxs {
                sgs_10_000.get(k);
            }
        })
    });
}

fn remove_bench(c: &mut Criterion) {
    let rtd_100 = RandTestData::new(100);
    let rtd_1_000 = RandTestData::new(1_000);
    let rtd_10_000 = RandTestData::new(10_000);

    let mut sgs_100 = SGSet::new();
    for k in &rtd_100.keys {
        sgs_100.insert(*k);
    }

    let mut sgs_1_000 = SGSet::new();
    for k in &rtd_100.keys {
        sgs_1_000.insert(*k);
    }

    let mut sgs_10_000 = SGSet::new();
    for k in &rtd_10_000.keys {
        sgs_10_000.insert(*k);
    }

    c.bench_function("sgs_remove_100", |b| {
        b.iter(|| {
            for k in &rtd_100.get_idxs {
                sgs_100.remove(k);
            }
        })
    });

    c.bench_function("sgs_remove_1_000", |b| {
        b.iter(|| {
            for k in &rtd_1_000.get_idxs {
                sgs_1_000.remove(k);
            }
        })
    });

    c.bench_function("sgs_remove_10_000", |b| {
        b.iter(|| {
            for k in &rtd_10_000.get_idxs {
                sgs_10_000.remove(k);
            }
        })
    });
}
// Runner --------------------------------------------------------------------------------------------------------------

criterion_group!(benches, insert_bench, get_bench, remove_bench);
criterion_main!(benches);