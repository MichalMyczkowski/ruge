use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use rand::{rngs::StdRng, SeedableRng};

pub fn string_to_rng(seed: String) -> StdRng {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    hasher.finish();
    StdRng::seed_from_u64(hasher.finish())
}
