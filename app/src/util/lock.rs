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

#[cfg(test)]
mod tests {
    use std::sync::{Arc, RwLock};

    use super::{safe_read, safe_write};

    fn get_lock() -> Arc<RwLock<i32>> {
        Arc::new(RwLock::new(1))
    }

    async fn poison_lock(lock: Arc<RwLock<i32>>) {
        let _ = tokio::spawn(async move {
            safe_write(&lock, |_| {
                panic!("Poisoned the RwLock explicitly.");
            });
        })
        .await;
    }

    #[test]
    fn should_write_successfully() {
        let data = 100;
        let lock = get_lock();

        let result = safe_write(&lock, |mut guard| {
            *guard = data;
            2 * guard.clone()
        });

        assert!(result.is_some());
        assert_eq!(2 * lock.read().unwrap().clone(), result.unwrap());
        assert_eq!(data, lock.read().unwrap().clone());
    }

    #[tokio::test]
    async fn should_fail_to_write_for_poisoned_lock() {
        let data = 100;
        let lock = get_lock();

        poison_lock(lock.clone()).await;
        let result = safe_write(&lock, |mut guard| {
            *guard = data;
        });

        assert!(result.is_none());
    }

    #[test]
    fn should_read_successfully() {
        let lock = get_lock();

        let result = safe_read(&lock, |guard| 2 * guard.clone());

        assert!(result.is_some());
        assert_eq!(2 * lock.read().unwrap().clone(), result.unwrap());
    }

    #[tokio::test]
    async fn should_fail_to_read_for_poisoned_lock() {
        let lock = get_lock();

        poison_lock(lock.clone()).await;
        let result = safe_read(&lock, |guard| 2 * guard.clone());

        assert!(result.is_none());
    }
}
