mod queue_vec;

#[derive(Debug)]
struct FooNode;

#[cfg(test)]
mod tests {
    use crate::FooNode;
    use crate::queue_vec::QueueVec;

    #[test]
    fn single_threaded() {
        let mut spill_queue = QueueVec::<FooNode>::new(3);
        spill_queue.push(FooNode);
        spill_queue.push(FooNode);
        spill_queue.push(FooNode);
        spill_queue.push(FooNode);
        println!("{:?}", spill_queue.get(3));
        spill_queue.defrag();
        println!("{:?}", spill_queue.get(3));
    }
}
