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
    pub fn new() -> Self {
        Queue {
        length: 0,
        head: None,
        tail: None,
        }
    }

    pub fn enqueue(&mut self, item: T) {
        let node = Box::new(Node {
            value: item,
            next: None,
        });

        self.length += 1;

        if self.tail.is_none() {
            self.tail = Some(node.clone());
            self.head = Some(node);
            return;
        }
    }
}


