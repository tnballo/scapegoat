use std::iter::FromIterator;

use scapegoat::SGSet;
use rand::Rng;

// Random Test Data ----------------------------------------------------------------------------------------------------

pub struct RandTestData {
    pub keys: Vec<usize>,
    pub get_idxs: Vec<usize>,
    pub remove_idxs: Vec<usize>,
}

impl RandTestData {
    pub fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();

        RandTestData {
            keys: (0..size).map(|_| rng.gen()).collect(),
            get_idxs: (0..size).map(|_| rng.gen_range(0, size)).collect(),
            remove_idxs: (0..size).map(|_| rng.gen_range(0, size)).collect(),
        }
    }
}

// Init Random Test Data (Immutable, Global) ---------------------------------------------------------------------------

lazy_static:: lazy_static! {
    pub static ref RTD_100: RandTestData = RandTestData::new(100);
    pub static ref RTD_1_000: RandTestData = RandTestData::new(1_000);
    pub static ref RTD_10_000: RandTestData = RandTestData::new(10_000);
}

lazy_static:: lazy_static! {
    pub static ref SGS_100: SGSet<usize> = SGSet::from_iter(RTD_100.keys.clone());
    pub static ref SGS_1_000: SGSet<usize> = SGSet::from_iter(RTD_1_000.keys.clone());
    pub static ref SGS_10_000: SGSet<usize> = SGSet::from_iter(RTD_10_000.keys.clone());
}