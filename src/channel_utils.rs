use std::{any::type_name, fmt::Debug};

use log::{log_enabled, trace};
use tokio::sync::mpsc::{error::SendError as TokioSendError, UnboundedSender};

pub struct LoggingTx<T>
where
    T: Debug,
{
    tx: UnboundedSender<T>,
}

impl<T: Debug> Clone for LoggingTx<T> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<T> LoggingTx<T>
where
    T: Debug,
{
    pub fn attach(tx: UnboundedSender<T>) -> Self {
        Self { tx }
    }

    pub fn send(&self, message: T) -> Result<(), TokioSendError<T>> {
        if log_enabled!(log::Level::Trace) {
            trace!("{} {:?}", type_name::<T>(), &message);
        }
        self.tx.send(message)
    }
}
