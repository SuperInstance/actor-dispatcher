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

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
