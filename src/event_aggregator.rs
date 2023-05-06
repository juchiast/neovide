use std::{
    any::{type_name, Any, TypeId},
    collections::{hash_map::Entry, HashMap},
    fmt::Debug,
    sync::Mutex,
};

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

use crate::channel_utils::*;

lazy_static! {
    pub static ref EVENT_AGGREGATOR: EventAggregator = EventAggregator::default();
}

pub struct EventAggregator {
    inner: Mutex<Inner>,
}

impl EventAggregator {
    pub fn send<T: Any + Clone + Debug + Send>(&self, event: T) {
        self.inner.lock().unwrap().send(event)
    }

    pub fn register_event<T: Any + Clone + Debug + Send>(&self) -> UnboundedReceiver<T> {
        self.inner.lock().unwrap().register_event()
    }
}

struct Inner {
    parent_senders: HashMap<TypeId, Box<dyn Any + Send>>,
    unclaimed_receivers: HashMap<TypeId, Box<dyn Any + Send>>,
}

impl Default for EventAggregator {
    fn default() -> Self {
        Self {
            inner: Mutex::new(Inner {
                parent_senders: HashMap::new(),
                unclaimed_receivers: HashMap::new(),
            }),
        }
    }
}

impl Inner {
    fn send<T: Any + Clone + Debug + Send>(&mut self, event: T) {
        let type_id = TypeId::of::<T>();
        match self.parent_senders.entry(type_id) {
            Entry::Occupied(entry) => {
                let sender = entry.get();
                sender
                    .downcast_ref::<LoggingTx<T>>()
                    .unwrap()
                    .send(event)
                    .unwrap();
            }
            Entry::Vacant(entry) => {
                let (sender, receiver) = unbounded_channel();
                let logging_tx = LoggingTx::attach(sender, type_name::<T>().to_owned());
                logging_tx.send(event).unwrap();
                entry.insert(Box::new(logging_tx));
                self.unclaimed_receivers.insert(type_id, Box::new(receiver));
            }
        };
    }

    fn register_event<T: Any + Clone + Debug + Send>(&mut self) -> UnboundedReceiver<T> {
        let type_id = TypeId::of::<T>();

        if let Some(receiver) = self.unclaimed_receivers.remove(&type_id) {
            *receiver.downcast::<UnboundedReceiver<T>>().unwrap()
        } else {
            let (sender, receiver) = unbounded_channel();
            let logging_sender = LoggingTx::attach(sender, type_name::<T>().to_owned());

            if self
                .parent_senders
                .insert(type_id, Box::new(logging_sender))
                .is_some()
            {
                panic!("EventAggregator: type already registered");
            }

            receiver
        }
    }
}
