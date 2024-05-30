use std::sync::Arc;

use parking_lot::{Mutex, RwLock};

pub type ArcMutex<T> = Arc<Mutex<T>>;
pub type ArcRwLock<T> = Arc<RwLock<T>>;

pub fn arc_mutex_new<T>(obj: T) -> ArcMutex<T> {
    Arc::new(Mutex::new(obj))
}

pub fn arc_rw_lock_new<T>(obj: T) -> ArcRwLock<T> {
    Arc::new(RwLock::new(obj))
}
