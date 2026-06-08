// src/lib.rs

use crossbeam_channel::{Receiver, Sender, TryRecvError, TrySendError, bounded};
use tracing::{debug, error, trace};

pub struct ThreadSafeBuffer<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> ThreadSafeBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = bounded(capacity);
        Self { sender, receiver }
    }

    pub fn push(&self, item: T) {
        self.sender.send(item).unwrap();
        trace!("Element succesvol aan buffer toegevoegd");
    }

    pub fn try_push(&self, item: T) -> Result<(), T> {
        match self.sender.try_send(item) {
            Ok(_) => {
                trace!("Element succesvol via try_push toegevoegd");
                Ok(())
            }
            Err(TrySendError::Full(rejected_item)) => {
                debug!("Buffer vol tijdens try_push");
                Err(rejected_item)
            }
            Err(TrySendError::Disconnected(_)) => {
                error!("Buffer unreachable in try_push!");
                unreachable!()
            }
        }
    }

    pub fn pop(&self) -> T {
        let item = self.receiver.recv().unwrap();
        trace!("Element succesvol uit buffer gehaald");
        item
    }

    pub fn try_pop(&self) -> Option<T> {
        match self.receiver.try_recv() {
            Ok(item) => {
                trace!("Element succesvol via try_pop gehaald");
                Some(item)
            }
            Err(TryRecvError::Empty) => {
                trace!("Buffer leeg tijdens try_pop");
                None
            }
            Err(TryRecvError::Disconnected) => {
                error!("Buffer unreachable in try_pop!");
                unreachable!()
            }
        }
    }

    /// Geeft het huidige aantal elementen in de buffer terug.
    /// Let op: in een multi-threaded omgeving is dit een momentopname.
    pub fn len(&self) -> usize {
        self.sender.len()
    }

    /// Geeft `true` terug als de buffer op dit moment helemaal leeg is.
    pub fn is_empty(&self) -> bool {
        self.sender.is_empty()
    }
}

impl<T> Clone for ThreadSafeBuffer<T> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            receiver: self.receiver.clone(),
        }
    }
}
