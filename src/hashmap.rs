use std::{collections::HashMap, hash::Hash, sync::{Arc, Mutex}};

use crate::{get_value, set_value};

pub fn insert<T1, T2>(arc: &Arc<Mutex<HashMap<T1, T2>>>, k: &T1, value: &T2)
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


pub fn count<T1, T2>(arc: &Arc<Mutex<HashMap<T1, T2>>>) -> usize 
where
    T1: Clone,
    T2: Clone
{
    crate::get_value(&arc).unwrap_or(HashMap::new()).len()
}

pub fn get<T1, T2>(arc: &Arc<Mutex<HashMap<T1, T2>>>) -> HashMap<T1, T2>
where 
    T1: Clone,
    T2: Clone,
{
    get_value(&arc).unwrap_or(HashMap::new())
}

pub fn clear<T1, T2>(arc: &Arc<Mutex<HashMap<T1, T2>>>){
    set_value(&arc, HashMap::new())
}