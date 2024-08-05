use std::sync::{Arc, Mutex};
use rand::seq::SliceRandom;

use crate::{get_value, set_value};
pub fn remove<T>(arc: &Arc<Mutex<Vec<T>>>, value: &T)
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

pub fn concat<T>(arc: &Arc<Mutex<Vec<T>>>, value: &[T])
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

pub fn push<T>(arc: &Arc<Mutex<Vec<T>>>, value: &T)
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

pub fn random<T>(arc: &Arc<Mutex<Vec<T>>>) -> Option<T> 
where
    T: Clone,
{
    let mut rng = rand::thread_rng();
    let vec = get_value(&arc).unwrap_or(Vec::new());
    let item = vec.choose(&mut rng);
    match item {
        Some(item) => Some(item.to_owned()),
        None => None,
    }
}

pub fn count<T>(arc: &Arc<Mutex<Vec<T>>>) -> usize 
where
    T: Clone,
{
    get_value(&arc).unwrap_or(Vec::new()).len()
}

pub fn get<T>(arc: &Arc<Mutex<Vec<T>>>) -> Vec<T>
where 
    T: Clone,
{
    get_value(&arc).unwrap_or(Vec::new())
}

pub fn cut<T>(arc: &Arc<Mutex<Vec<T>>>, size: usize) -> Option<Vec<T>>
where
    T: Clone + Copy,
{
    match get_value(&arc){
        Some(mut vec)=>{
            let taken: Vec<T> = vec.iter().take(size).copied().collect();
            set_value(arc, vec.split_off(size));
            return Some(taken)
        },
        None=>None,
    }
}