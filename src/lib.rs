use std::{collections::{HashMap, HashSet}, hash::Hash, sync::{Arc, Mutex}};
pub fn get_arc<T>(data: T) -> Arc<Mutex<T>> {
    Arc::new(Mutex::new(data))
}

pub fn get_value<T>(data: &Arc<Mutex<T>>) -> Option<T> 
where
    T: Clone
{
    let arc_clone = Arc::clone(&data);
    if let Ok(guard) = arc_clone.lock() {
        let value = guard.clone();
        return Some(value)
    }
    None
}
pub fn set_value<T>(arc: &Arc<Mutex<T>>, value: T){
    if let Ok(mut guard) = arc.lock() {
        *guard = value;
    }
}
pub fn push_value<T>(arc: &Arc<Mutex<Vec<T>>>, value: &T)
where
    T: Clone,
{
    let mut guard = match arc.lock() {
        Ok(guard) => guard,
        Err(_) => {
            eprintln!("Failed to acquire lock on Arc<Mutex<Vec<T>>>");
            return;
        }
    };

    guard.push(value.to_owned());
}

pub fn put_value<T>(arc: &Arc<Mutex<HashSet<T>>>, value: T)
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

pub fn concat_value<T>(arc: &Arc<Mutex<Vec<T>>>, value: &[T])
where
    T: Clone,
{
    let mut guard = match arc.lock() {
        Ok(guard) => guard,
        Err(_) => {
            eprintln!("Failed to acquire lock on Arc<Mutex<Vec<T>>>");
            return;
        }
    };

    guard.extend_from_slice(value);
}

pub fn insert_value<T1, T2>(arc: &Arc<Mutex<HashMap<T1, T2>>>, k: &T1, value: &T2)
where
    T1: Clone + Eq + Hash,
    T2: Clone
{
    let mut guard = match arc.lock() {
        Ok(guard) => guard,
        Err(_) => {
            eprintln!("Failed to acquire lock on Arc<Mutex<HashMap<T>>>");
            return;
        }
    };

    guard.insert(k.to_owned(), value.to_owned());
}
pub fn add_value<T>(arc: &Arc<Mutex<T>>, value: T)
where
    T: Clone + std::ops::AddAssign,
{
    let mut guard = match arc.lock() {
        Ok(guard) => guard,
        Err(_) => {
            eprintln!("Failed to acquire lock on T");
            return;
        }
    };
    *guard += value;
}

pub fn sub_value<T>(arc: &Arc<Mutex<T>>, value: T)
where
    T: Clone + std::ops::SubAssign,
{
    let mut guard = match arc.lock() {
        Ok(guard) => guard,
        Err(_) => {
            eprintln!("Failed to acquire lock on T");
            return;
        }
    };
    *guard -= value;
}

pub fn remove_value<T>(arc: &Arc<Mutex<Vec<T>>>, value: &T)
where
    T: Eq + PartialEq
{
    let mut guard = match arc.lock() {
        Ok(guard) => guard,
        Err(_) => {
            eprintln!("Failed to acquire lock on T");
            return;
        }
    };
    if let Some(index ) = guard.iter().position(|s| s == value) {
        guard.remove(index);
    }
}