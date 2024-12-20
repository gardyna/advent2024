pub mod template;

// Use this file to add helper functions and additional modules.
use std::hash::Hash;
use rustc_hash::FxHashMap;
use num_traits::Float;

pub fn count_elements<'a, T: Hash + Eq + Copy + Clone + 'a>(
    values: impl IntoIterator<Item = &'a T>,
) -> FxHashMap<T, usize> {
    let mut ret: FxHashMap<T, usize> = Default::default();
    for v in values.into_iter() {
        *ret.entry(*v).or_default() += 1;
    }
    ret
}

pub fn is_integer<T: Float>(num: T) -> bool {
    num.fract() == T::zero()
}