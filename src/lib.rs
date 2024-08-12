use std::{collections::{HashMap, HashSet}, hash::Hash, sync::{Arc, Mutex}};
pub mod vec;
pub mod hashset;
pub mod int;
pub mod hashmap;
pub mod string;
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
    vec::push(arc, value)
}

pub fn put_value<T>(arc: &Arc<Mutex<HashSet<T>>>, value: T)
where
    T: Eq + Hash,
{
    hashset::put(arc, value)
}

pub fn concat_value<T>(arc: &Arc<Mutex<Vec<T>>>, value: &[T])
where
    T: Clone,
{
    vec::concat(arc, value)
}

pub fn insert_value<T1, T2>(arc: &Arc<Mutex<HashMap<T1, T2>>>, k: &T1, value: &T2)
where
    T1: Clone + Eq + Hash,
    T2: Clone
{
    hashmap::insert(arc, k, value);
}
pub fn add_value<T>(arc: &Arc<Mutex<T>>, value: T)
where
    T: Clone + std::ops::AddAssign,
{
    int::add(arc, value)
}

pub fn sub_value<T>(arc: &Arc<Mutex<T>>, value: T)
where
    T: Clone + std::ops::SubAssign,
{
    int::sub(arc, value)
}

pub fn remove_value<T>(arc: &Arc<Mutex<Vec<T>>>, value: &T)
where
    T: Eq + PartialEq
{
    vec::remove(arc, value)
}

pub fn remove_value_hashset<T>(arc: &Arc<Mutex<HashSet<T>>>, value: &T)
where
    T: Eq + PartialEq + Hash
{
    hashset::remove(arc, value)
}