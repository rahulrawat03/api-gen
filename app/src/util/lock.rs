use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use tracing::{error, info};

pub fn safe_write<T, F, R>(lock: &RwLock<T>, operation: F) -> Option<R>
where
    F: FnOnce(RwLockWriteGuard<T>) -> R,
{
    match lock.write() {
        Ok(guard) => {
            info!("Succesfully acquired write lock.");
            Some(operation(guard))
        }
        Err(_) => {
            error!("Failed to acquire write lock.");
            None
        }
    }
}

pub fn safe_read<T, F, R>(lock: &RwLock<T>, operation: F) -> Option<R>
where
    F: FnOnce(RwLockReadGuard<T>) -> R,
{
    match lock.read() {
        Ok(guard) => {
            info!("Successfully acquired read lock.");
            Some(operation(guard))
        }
        Err(_) => {
            error!("Failed to acquire read lock.");
            None
        }
    }
}
