#[derive(Clone, Debug)]
struct StackNode<T> {
    val: T,
    next: Option<Box<StackNode<T>>>,
}
#[derive(Debug)]
struct Stack<T> {
    top: Option<Box<StackNode<T>>>,
}
impl<T> StackNode<T> {
    fn new(val: T) -> StackNode<T> {
        StackNode { val, next: None }
    }
}
impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { top: None }
    }
    fn push(&mut self, val: T) {
        let mut node = StackNode::new(val);
        node.next = self.top.take();
        self.top = Some(Box::new(node));
    }
    fn pop(&mut self) -> Option<T> {
        let top = self.top.take();
        match top {
            None => None,
            Some(mut x) => {
                self.top = x.next.take();
                Some(x.val)
            }
        }
    }
}

fn main() {
    #[derive(Debug)]
    struct StackTest {
        a: i32,
    }
    let x = StackTest { a: 1 };
    let y = StackTest { a: 2 };
    let z = StackTest { a: 3 };

    let mut s = Stack::new();
    s.push(&x);
    s.push(&y);
    s.push(&z);
    println!("{:?}", s);
    println!("pop: {:?}", s.pop());
    println!("After pop: {:?}", s);
}
