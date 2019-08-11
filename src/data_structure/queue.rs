#[derive(Debug)]
struct Queue<T> {
    q: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue { q: Vec::new() }
    }
    fn push(&mut self, v: T) {
        self.q.push(v)
    }
    fn pop(&mut self) -> Option<T> {
        match self.q.len() {
            0 => None,
            _ => Some(self.q.remove(0)),
        }
    }
}

pub fn test() {
    println!("Running Queue Receipt:");
    let mut queue = Queue::new();
    println!("{:?}", queue);
    println!("{:?}", queue.pop());
    queue.push(1);
    println!("{:?}", queue);
    println!("Pop: {:?}", queue.pop().unwrap());
    println!("After pop: {:?}", queue);
}
