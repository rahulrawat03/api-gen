use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub fn safe_write<T, F, R>(lock: &RwLock<T>, operation: F) -> Option<R>
where
    F: FnOnce(RwLockWriteGuard<T>) -> R,
{
    match lock.write() {
        Ok(guard) => Some(operation(guard)),
        Err(_) => None,
    }
}

pub fn safe_read<T, F, R>(lock: &RwLock<T>, operation: F) -> Option<R>
where
    F: FnOnce(RwLockReadGuard<T>) -> R,
{
    match lock.read() {
        Ok(guard) => Some(operation(guard)),
        Err(_) => None,
    }
}
