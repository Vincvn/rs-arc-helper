use std::{collections:: HashSet, hash::Hash, sync::{Arc, Mutex}};
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