#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}


#[derive(Debug)]
pub struct Queue<T> {
    length: usize,
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}

impl<T> Queue<T> {
    pub fn new() - Self {
        Queue {
        length: 0,
        head: None,
        tail: None,
        }
    }
}

