use bumpalo_herd::{Herd, Member};
use fixed_vec::FixedVec;
use std::cell::OnceCell;
use std::ptr;
use std::ptr::null_mut;
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicPtr, AtomicUsize};

#[derive(Debug, Default)]
pub struct QueueVec<T> {
    main: FixedVec<T>,
    queue: Queue<T>,
}

impl<T> QueueVec<T> {
    pub fn push(&self, item: T) -> usize {
        match self.main.push(item) {
            Ok(idx) => idx,
            Err(item) => self.queue.push(item),
        }
    }

    pub fn realloc(&mut self) {
        // TODO: Add a function to QueueVec that resizes to fit a slice.
    }
}

struct QueueItem<T> {
    value: T,
    prev: AtomicPtr<QueueItem<T>>,
}

#[derive(Debug, Default)]
struct Queue<T> {
    herd: Herd,
    len: AtomicUsize,
    tail: AtomicPtr<QueueItem<T>>,
}

thread_local! {
    static BUMP: OnceCell<Member<'static>> = OnceCell::new();
}

impl<T> Queue<T> {
    fn push(&self, item: T) -> usize {
        let idx = self.len.fetch_add(1, Relaxed);
        let queue_item = QueueItem {
            value: item,
            prev: AtomicPtr::new(self.tail.load(Relaxed)),
        };
        let bump = BUMP.with(|b| b.get_or_init(move || self.herd.get()));
        self.tail.store(bump.alloc(queue_item), Relaxed);
        idx
    }

    fn into_vec(self) -> Vec<T> {
        let len = self.len.fetch_add(1, Relaxed);
        let mut vec = Vec::with_capacity(len);
        let mut queue_item = self.tail.load(Relaxed);
        while queue_item != null_mut() {
            // SAFETY: self is consumed so this will not be read again.
            let owned_queue_item = unsafe { ptr::read(queue_item) };
            queue_item = owned_queue_item.prev.into_inner();
            vec.push(owned_queue_item.value);
        }
        vec
    }
}
