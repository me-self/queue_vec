use fixed_vec::FixedVec;
use std::mem::take;

#[derive(Debug)]
pub struct QueueVec<T> {
    vec: FixedVec<T>,
    queue: boxcar::Vec<T>,
}

impl<T> QueueVec<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            vec: FixedVec::new(capacity),
            queue: boxcar::Vec::new(),
        }
    }

    pub fn push(&self, item: T) -> usize {
        self.vec
            .push(item)
            .unwrap_or_else(|item| self.queue.push(item))
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index).or_else(|| {
            println!("Index is in queue.");
            // It's ok to use `reserved_len` here since we aren't reading elements from the
            // vec itself.
            self.queue.get(index - self.vec.reserved_len())
        })
    }

    pub fn defrag(&mut self) {
        let queue = take(&mut self.queue);

        let new_items = queue.into_iter();
        let total_len = self.vec.reserved_len() + new_items.len();
        let new_length = total_len.next_power_of_two();

        self.vec.realloc(new_length);
        self.vec.extend(new_items);
    }
}
