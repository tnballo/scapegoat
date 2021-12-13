use std::collections::BTreeSet;
use std::iter::FromIterator;

use criterion::{criterion_group, criterion_main, Criterion};
use scapegoat::SgSet;

mod test_data;
use test_data::{
    SEQ_100, SEQ_10_000, SEQ_1_000, SGS_100_SEQ, SGS_10_000_SEQ, SGS_1_000_SEQ, STD_100_SEQ,
    STD_10_000_SEQ, STD_1_000_SEQ,
};

// Benches -------------------------------------------------------------------------------------------------------------

fn bench_insert(c: &mut Criterion) {
    // SGS vs STD 100 --------------------------------------------------------------------------------------------------

    c.bench_function("sgs_insert_100_seq", |b| {
        b.iter(|| {
            let mut sgs = SgSet::<_, 100>::new();
            for k in &SEQ_100.keys {
                sgs.insert(*k);
            }
        })
    });

    c.bench_function("std_insert_100_seq", |b| {
        b.iter(|| {
            let mut std = BTreeSet::new();
            for k in &SEQ_100.keys {
                std.insert(*k);
            }
        })
    });

    // SGS vs STD 1_000 ------------------------------------------------------------------------------------------------

    c.bench_function("sgs_insert_1_000_seq", |b| {
        b.iter(|| {
            let mut sgs = SgSet::<_, 1_000>::new();
            for k in &SEQ_1_000.keys {
                sgs.insert(*k);
            }
        })
    });

    c.bench_function("std_insert_1_000_seq", |b| {
        b.iter(|| {
            let mut std = BTreeSet::new();
            for k in &SEQ_1_000.keys {
                std.insert(*k);
            }
        })
    });

    // SGS vs STD 10_000 -----------------------------------------------------------------------------------------------

    c.bench_function("sgs_insert_10_000_seq", |b| {
        b.iter(|| {
            let mut sgs = SgSet::<_, 10_000>::new();
            for k in &SEQ_10_000.keys {
                sgs.insert(*k);
            }
        })
    });

    c.bench_function("std_insert_10_000_seq", |b| {
        b.iter(|| {
            let mut std = BTreeSet::new();
            for k in &SEQ_10_000.keys {
                std.insert(*k);
            }
        })
    });
}

fn bench_get(c: &mut Criterion) {
    // SGS vs STD 100 --------------------------------------------------------------------------------------------------

    c.bench_function("sgs_get_100_seq", |b| {
        b.iter(|| {
            for k in &SEQ_100.get_idxs {
                let _ = &SGS_100_SEQ.get(k);
            }
        })
    });

    c.bench_function("std_get_100_seq", |b| {
        b.iter(|| {
            for k in &SEQ_100.get_idxs {
                let _ = &STD_100_SEQ.get(k);
            }
        })
    });

    // SGS vs STD 1_000 ------------------------------------------------------------------------------------------------

    c.bench_function("sgs_get_1_000_seq", |b| {
        b.iter(|| {
            for k in &SEQ_1_000.get_idxs {
                let _ = &SGS_1_000_SEQ.get(k);
            }
        })
    });

    c.bench_function("std_get_1_000_seq", |b| {
        b.iter(|| {
            for k in &SEQ_1_000.get_idxs {
                let _ = &STD_1_000_SEQ.get(k);
            }
        })
    });

    // SGS vs STD 10_000 -----------------------------------------------------------------------------------------------

    c.bench_function("sgs_get_10_000_seq", |b| {
        b.iter(|| {
            for k in &SEQ_10_000.get_idxs {
                let _ = &SGS_10_000_SEQ.get(k);
            }
        })
    });

    c.bench_function("std_get_10_000_seq", |b| {
        b.iter(|| {
            for k in &SEQ_10_000.get_idxs {
                let _ = &STD_10_000_SEQ.get(k);
            }
        })
    });
}

fn bench_remove(c: &mut Criterion) {
    let mut sgs_100: SgSet<usize, 100> = SgSet::from_iter(SEQ_100.keys.clone());
    let mut sgs_1_000: SgSet<usize, 1_000> = SgSet::from_iter(SEQ_1_000.keys.clone());
    let mut sgs_10_000: SgSet<usize, 10_000> = SgSet::from_iter(SEQ_10_000.keys.clone());

    let mut std_100: BTreeSet<usize> = BTreeSet::from_iter(SEQ_100.keys.clone());
    let mut std_1_000: BTreeSet<usize> = BTreeSet::from_iter(SEQ_1_000.keys.clone());
    let mut std_10_000: BTreeSet<usize> = BTreeSet::from_iter(SEQ_10_000.keys.clone());

    // SGS vs STD 100 --------------------------------------------------------------------------------------------------

    c.bench_function("sgs_remove_100_seq", |b| {
        b.iter(|| {
            for k in &SEQ_100.remove_idxs {
                sgs_100.remove(k);
            }
        })
    });

    c.bench_function("std_remove_100_seq", |b| {
        b.iter(|| {
            for k in &SEQ_100.remove_idxs {
                std_100.remove(k);
            }
        })
    });

    // SGS vs STD 1_000 ------------------------------------------------------------------------------------------------

    c.bench_function("sgs_remove_1_000_seq", |b| {
        b.iter(|| {
            for k in &SEQ_1_000.remove_idxs {
                sgs_1_000.remove(k);
            }
        })
    });

    c.bench_function("std_remove_1_000_seq", |b| {
        b.iter(|| {
            for k in &SEQ_1_000.remove_idxs {
                std_1_000.remove(k);
            }
        })
    });

    // SGS vs STD 10_000 -----------------------------------------------------------------------------------------------

    c.bench_function("sgs_remove_10_000_seq", |b| {
        b.iter(|| {
            for k in &SEQ_10_000.remove_idxs {
                sgs_10_000.remove(k);
            }
        })
    });

    c.bench_function("std_remove_10_000_seq", |b| {
        b.iter(|| {
            for k in &SEQ_10_000.remove_idxs {
                std_10_000.remove(k);
            }
        })
    });
}

// Runner --------------------------------------------------------------------------------------------------------------

criterion_group!(benches, bench_insert, bench_get, bench_remove);
criterion_main!(benches);
