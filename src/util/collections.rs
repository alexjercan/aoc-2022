use std::collections::HashSet;

pub fn unique<T>(collection: &Vec<T>) -> Vec<T>
where
    T: Eq + std::hash::Hash + Copy,
{
    Vec::from_iter(
        HashSet::<T>::from_iter(collection.iter().copied())
            .iter()
            .copied(),
    )
}
