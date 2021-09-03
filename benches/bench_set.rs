use std::iter::FromIterator;
use std::collections::BTreeSet;

use criterion::{criterion_group, criterion_main, Criterion};
use scapegoat::SGSet;

mod rtd;
use rtd::{RTD_100, RTD_10_000, RTD_1_000, SGS_100, SGS_10_000, SGS_1_000, STD_100, STD_1_000, STD_10_000};

// Benches -------------------------------------------------------------------------------------------------------------

fn bench_insert(c: &mut Criterion) {

    // SGS vs STD 100 --------------------------------------------------------------------------------------------------

    c.bench_function("sgs_insert_100", |b| {
        b.iter(|| {
            let mut sgs = SGSet::new();
            for k in &RTD_100.keys {
                sgs.insert(*k);
            }
        })
    });

    c.bench_function("std_insert_100", |b| {
        b.iter(|| {
            let mut std = SGSet::new();
            for k in &RTD_100.keys {
                std.insert(*k);
            }
        })
    });

    // SGS vs STD 1_000 ------------------------------------------------------------------------------------------------

    c.bench_function("sgs_insert_1_000", |b| {
        b.iter(|| {
            let mut sgs = SGSet::new();
            for k in &RTD_1_000.keys {
                sgs.insert(*k);
            }
        })
    });

    c.bench_function("std_insert_1_000", |b| {
        b.iter(|| {
            let mut std = BTreeSet::new();
            for k in &RTD_1_000.keys {
                std.insert(*k);
            }
        })
    });

    // SGS vs STD 10_000 -----------------------------------------------------------------------------------------------

    c.bench_function("sgs_insert_10_000", |b| {
        b.iter(|| {
            let mut sgs = SGSet::new();
            for k in &RTD_10_000.keys {
                sgs.insert(*k);
            }
        })
    });

    c.bench_function("std_insert_10_000", |b| {
        b.iter(|| {
            let mut std = BTreeSet::new();
            for k in &RTD_10_000.keys {
                std.insert(*k);
            }
        })
    });
}

fn bench_get(c: &mut Criterion) {

    // SGS vs STD 100 --------------------------------------------------------------------------------------------------

    c.bench_function("sgs_get_100", |b| {
        b.iter(|| {
            for k in &RTD_100.get_idxs {
                &SGS_100.get(k);
            }
        })
    });

    c.bench_function("std_get_100", |b| {
        b.iter(|| {
            for k in &RTD_100.get_idxs {
                &STD_100.get(k);
            }
        })
    });

    // SGS vs STD 1_000 ------------------------------------------------------------------------------------------------

    c.bench_function("sgs_get_1_000", |b| {
        b.iter(|| {
            for k in &RTD_1_000.get_idxs {
                &SGS_1_000.get(k);
            }
        })
    });

    c.bench_function("std_get_1_000", |b| {
        b.iter(|| {
            for k in &RTD_1_000.get_idxs {
                &STD_1_000.get(k);
            }
        })
    });

    // SGS vs STD 10_000 -----------------------------------------------------------------------------------------------

    c.bench_function("sgs_get_10_000", |b| {
        b.iter(|| {
            for k in &RTD_10_000.get_idxs {
                &SGS_10_000.get(k);
            }
        })
    });

    c.bench_function("std_get_10_000", |b| {
        b.iter(|| {
            for k in &RTD_10_000.get_idxs {
                &STD_10_000.get(k);
            }
        })
    });
}

fn bench_remove(c: &mut Criterion) {
    let mut sgs_100: SGSet<usize> = SGSet::from_iter(RTD_100.keys.clone());
    let mut sgs_1_000: SGSet<usize> = SGSet::from_iter(RTD_1_000.keys.clone());
    let mut sgs_10_000: SGSet<usize> = SGSet::from_iter(RTD_10_000.keys.clone());

    let mut std_100: BTreeSet<usize> = BTreeSet::from_iter(RTD_100.keys.clone());
    let mut std_1_000: BTreeSet<usize> = BTreeSet::from_iter(RTD_1_000.keys.clone());
    let mut std_10_000: BTreeSet<usize> = BTreeSet::from_iter(RTD_10_000.keys.clone());

    // SGS vs STD 100 --------------------------------------------------------------------------------------------------

    c.bench_function("sgs_remove_100", |b| {
        b.iter(|| {
            for k in &RTD_100.remove_idxs {
                sgs_100.remove(k);
            }
        })
    });

    c.bench_function("std_remove_100", |b| {
        b.iter(|| {
            for k in &RTD_100.remove_idxs {
                std_100.remove(k);
            }
        })
    });

    // SGS vs STD 1_000 ------------------------------------------------------------------------------------------------

    c.bench_function("sgs_remove_1_000", |b| {
        b.iter(|| {
            for k in &RTD_1_000.remove_idxs {
                sgs_1_000.remove(k);
            }
        })
    });

    c.bench_function("std_remove_1_000", |b| {
        b.iter(|| {
            for k in &RTD_1_000.remove_idxs {
                std_1_000.remove(k);
            }
        })
    });

    // SGS vs STD 10_000 -----------------------------------------------------------------------------------------------

    c.bench_function("sgs_remove_10_000", |b| {
        b.iter(|| {
            for k in &RTD_10_000.remove_idxs {
                sgs_10_000.remove(k);
            }
        })
    });

    c.bench_function("std_remove_10_000", |b| {
        b.iter(|| {
            for k in &RTD_10_000.remove_idxs {
                std_10_000.remove(k);
            }
        })
    });
}

// Runner --------------------------------------------------------------------------------------------------------------

criterion_group!(benches, bench_insert, bench_get, bench_remove);
criterion_main!(benches);
