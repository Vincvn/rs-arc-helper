use std::sync::{Arc, Mutex};

pub fn add<T>(arc: &Arc<Mutex<T>>, value: T)
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

pub fn sub<T>(arc: &Arc<Mutex<T>>, value: T)
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