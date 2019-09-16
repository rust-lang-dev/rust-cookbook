use ansi_term::Colour;
use core::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Clone, Debug)]
struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    pub size: u32,
}
impl<T> Node<T> {
    fn new(val: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: val,
            next: None,
        }))
    }
}
impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            head: None,
            tail: None,
            size: 0,
        }
    }
    fn push(&mut self, val: T) {
        let node = Node::new(val);
        match self.tail.take() {
            None => {
                self.head = Some(node.clone());
            }
            Some(current) => {
                current.borrow_mut().next = Some(node.clone());
            }
        }
        self.tail = Some(node);
        self.size += 1;
    }
    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.size -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Error during unwrap head")
                .into_inner()
                .value
        })
    }
}

pub fn test() {
    println!(
        "{}",
        Colour::Green.bold().paint("Running Linked List Receipt:")
    );
    #[derive(Debug)]
    struct LinkedListTest {
        a: i32,
    }
    let x = LinkedListTest { a: 1 };
    let y = LinkedListTest { a: 2 };
    let z = LinkedListTest { a: 3 };

    let mut s = LinkedList::new();
    s.push(&x);
    s.pop();
    println!("{:?}", s);
    s.push(&y);
    s.push(&z);
    println!("{:?}", s);
}
