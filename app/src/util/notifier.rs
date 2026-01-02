use std::sync::RwLock;

use tokio::sync::Notify;

use crate::util::lock::{safe_read, safe_write};

#[derive(Clone)]
pub enum NotifierState {
    Pending,
    Notified,
    Corrupted,
}

#[derive(Debug)]
pub enum NotificationError {
    AlreadyFired,
    NoAvailableData,
    LockAcquisition,
}

pub struct Notifier<T> {
    notify: Notify,
    data: RwLock<Option<T>>,
    state: RwLock<NotifierState>,
}

impl<T> Notifier<T> {
    pub fn new() -> Self {
        Self {
            notify: Notify::new(),
            data: RwLock::new(None),
            state: RwLock::new(NotifierState::Pending),
        }
    }

    pub fn notify(&self, data: T) -> Result<(), NotificationError> {
        if !self.can_notify() {
            return Err(NotificationError::AlreadyFired);
        }

        let result = safe_write(&self.data, |mut guard| {
            *guard = Some(data);
        });
        safe_write(&self.state, |mut guard| {
            match result {
                Some(_) => *guard = NotifierState::Notified,
                None => *guard = NotifierState::Corrupted,
            };
        });

        self.notify.notify_waiters();

        Ok(())
    }

    fn can_notify(&self) -> bool {
        matches!(self.get_state(), NotifierState::Pending)
    }

    fn get_state(&self) -> NotifierState {
        safe_read(&self.state, |guard| guard.clone())
            .unwrap_or(NotifierState::Corrupted)
    }

    pub async fn await_notification(&self) -> Result<T, NotificationError> {
        match self.get_state() {
            NotifierState::Pending => {
                self.notify.notified().await;
                Box::pin(self.await_notification()).await
            }
            NotifierState::Notified => {
                let data = safe_write(&self.data, |mut guard| guard.take());

                if let Some(Some(data)) = data {
                    Ok(data)
                } else {
                    Err(NotificationError::NoAvailableData)
                }
            }
            NotifierState::Corrupted => Err(NotificationError::NoAvailableData),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::util::{
        lock::{safe_read, safe_write},
        notifier::{NotificationError, NotifierState},
    };

    use super::Notifier;

    #[tokio::test]
    async fn should_notify_successfully() {
        let data = 1;
        let notifier = Arc::new(Notifier::new());

        let notifier_clone = notifier.clone();
        let handle = tokio::spawn(async move {
            let _ = notifier.notify(data);
        });

        let result = notifier_clone.await_notification().await;
        assert!(result.is_ok());
        assert_eq!(data, result.unwrap());

        let _ = handle.await;
    }

    #[tokio::test]
    async fn should_not_allow_multiple_notifications() {
        let data = 1;
        let notifier = Notifier::new();

        let _first_result = notifier.notify(data);
        let second_result = notifier.notify(data);

        assert!(second_result.is_err());
        assert!(matches!(
            second_result,
            Err(NotificationError::AlreadyFired),
        ));
    }

    #[tokio::test]
    async fn should_get_result_immediately_if_notification_already_fired() {
        let data = 1;
        let notifier = Notifier::new();

        let _ = notifier.notify(data);

        let notification_result = notifier.await_notification().await;
        assert!(notification_result.is_ok());
        assert_eq!(data, notification_result.unwrap());
    }

    #[tokio::test]
    async fn should_become_corrupted_if_data_cannot_be_saved() {
        let data = 1;
        let notifier = Arc::new(Notifier::new());

        poison_data_lock(notifier.clone()).await;
        let _ = notifier.notify(data);

        safe_read(&notifier.state, |guard| {
            assert!(matches!(guard.clone(), NotifierState::Corrupted));
        });
    }

    #[tokio::test]
    async fn should_fail_with_unavailable_data_for_corrupted_notifier() {
        let data = 1;
        let notifier = Arc::new(Notifier::new());

        poison_data_lock(notifier.clone()).await;
        let _ = notifier.notify(data);

        let result = notifier.await_notification().await;
        assert!(result.is_err());
        assert!(matches!(result, Err(NotificationError::NoAvailableData)));
    }

    async fn poison_data_lock<T: Send + Sync + 'static>(
        notifier: Arc<Notifier<T>>,
    ) {
        let _ = tokio::spawn(async move {
            safe_write(&notifier.data, |_| {
                panic!("Poisoned the RwLock explicitly.");
            });
        })
        .await;
    }
}
