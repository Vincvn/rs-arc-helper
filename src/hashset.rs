use std::{collections:: HashSet, hash::Hash, sync::{Arc, Mutex}};
use rand::seq::SliceRandom;
pub fn remove<T>(arc: &Arc<Mutex<HashSet<T>>>, value: &T)
where
    T: Eq + PartialEq + Hash
{
    let mut guard = match arc.lock() {
        Ok(guard) => guard,
        Err(_) => {
            eprintln!("Failed to acquire lock on T");
            return;
        }
    };
    guard.remove(value);
}

pub fn put<T>(arc: &Arc<Mutex<HashSet<T>>>, value: T)
where
    T: Eq + Hash,
{
    let mut guard = match arc.lock() {
        Ok(guard) => guard,
        Err(_) => {
            eprintln!("Failed to acquire lock on Arc<Mutex<Vec<T>>>");
            return;
        }
    };

    guard.insert(value);
}
pub fn random<T>(arc: &Arc<Mutex<HashSet<T>>>) -> Option<T>
where
    T: Eq + Hash + Clone, 
{
    let items = crate::get_value(&arc).unwrap_or(HashSet::new());
    let vec: Vec<T> = items.iter().map(|item|item.to_owned()).collect();
    let mut rng = rand::thread_rng();
    let item = vec.choose(&mut rng);
    match item {
        Some(value) => Some(value.to_owned()),
        None=>None,
    }
}