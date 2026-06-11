//! actor-dispatcher — dispatches messages to actors with back-pressure.

use std::collections::VecDeque;

/// A bounded dispatcher queue with simple back-pressure.
#[derive(Debug)]
pub struct Dispatcher<T> {
    queue: VecDeque<T>,
    capacity: usize,
}

impl<T> Dispatcher<T> {
    pub fn new(capacity: usize) -> Self {
        Self { queue: VecDeque::with_capacity(capacity), capacity }
    }

    pub fn try_push(&mut self, msg: T) -> Result<(), T> {
        if self.queue.len() >= self.capacity {
            Err(msg)
        } else {
            self.queue.push_back(msg);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backpressure() {
        let mut d: Dispatcher<i32> = Dispatcher::new(2);
        assert!(d.try_push(1).is_ok());
        assert!(d.try_push(2).is_ok());
        assert_eq!(d.try_push(3), Err(3));
        assert_eq!(d.pop(), Some(1));
        assert!(d.try_push(3).is_ok());
    }
}
