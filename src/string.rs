use std::sync::{Arc, Mutex};
pub fn get(arc: &Arc<Mutex<String>>) -> String{
    crate::get_value(arc).unwrap_or(String::new())
}