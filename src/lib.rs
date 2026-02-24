mod queue_vec;

#[cfg(test)]
mod tests {
    use crate::queue_vec::QueueVec;

    #[test]
    fn single_threaded() {
        let mut queue_vec = QueueVec::default();
        queue_vec.push('h');
        queue_vec.push('e');
        queue_vec.push('l');
        queue_vec.push('l');
        queue_vec.push('o');
        queue_vec.push('w');
        queue_vec.push('o');
        queue_vec.push('r');
        queue_vec.push('l');
        queue_vec.push('d');
    }
}
