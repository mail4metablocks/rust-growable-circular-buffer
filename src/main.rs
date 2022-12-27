use std::ops::{Deref, DerefMut};

struct CircularBuffer {
    storage: Vec<u8>,
    start: usize,
    end: usize,
}

impl CircularBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            storage: Vec::with_capacity(capacity),
            start: 0,
            end: 0,
        }
    }

    fn push(&mut self, element: u8) {
        if self.storage.len() == self.storage.capacity() {
            // Grow the storage by allocating a new, larger Vec and copying the data over
            let new_capacity = self.storage.capacity() * 2;
            let mut new_storage = Vec::with_capacity(new_capacity);
            let mut idx = self.start;
            while idx != self.end {
                new_storage.push(self.storage[idx]);
                idx = (idx + 1) % self.storage.capacity();
            }
            self.storage = new_storage;
            self.start = 0;
            self.end = self.storage.len();
        }
        self.storage[self.end] = element;
        self.end = (self.end + 1) % self.storage.capacity();
    }

    fn pop(&mut self) -> Option<u8> {
        if self.start == self.end {
            return None;
        }
        let element = self.storage[self.start];
        self.start = (self.start + 1) % self.storage.capacity();
        Some(element)
    }
}

impl Deref for CircularBuffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.storage[self.start..self.end]
    }
}

impl DerefMut for CircularBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage[self.start..self.end]
    }
}
