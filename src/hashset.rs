use std::{collections:: HashSet, hash::Hash, sync::{Arc, Mutex}};
use rand::seq::SliceRandom;

use crate::{get_value, set_value};
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
pub fn to_vec<T>(arc: &Arc<Mutex<HashSet<T>>>) -> Vec<T>
where
    T: Clone
{
    let vec: Vec<T> = get(&arc).iter().map(|item|item.to_owned()).collect();
    vec
}
pub fn random<T>(arc: &Arc<Mutex<HashSet<T>>>) -> Option<T>
where
    T: Eq + Hash + Clone, 
{
    let vec: Vec<T> = to_vec(&arc);
    let mut rng = rand::thread_rng();
    let item = vec.choose(&mut rng);
    match item {
        Some(value) => Some(value.to_owned()),
        None=>None,
    }
}

pub fn count<T>(arc: &Arc<Mutex<HashSet<T>>>) -> usize 
where
    T: Clone,
{
    crate::get_value(&arc).unwrap_or(HashSet::new()).len()
}

pub fn get<T>(arc: &Arc<Mutex<HashSet<T>>>) -> HashSet<T>
where 
    T: Clone,
{
    get_value(&arc).unwrap_or(HashSet::new())
}

pub fn cut<T>(arc: &Arc<Mutex<HashSet<T>>>, size: usize) -> Option<Vec<T>>
where
    T: Clone + Copy + Eq + Hash,
{
    match get_value(&arc){
        Some(mut data)=>{
            let taken: Vec<T> = data.iter().map(|item|item.to_owned()).collect::<Vec<T>>().iter().take(size).copied().collect();
            taken.clone().into_iter().for_each(|f|{
                data.remove(&f);
            });
            set_value(&arc, data);
            return Some(taken)
        },
        None=>None,
    }
}