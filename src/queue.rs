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

        if let Some(ref mut tail) = self.tail {
            tail.next = Some(node);
            self.tail = tail.next.clone();
        }
        
    }
    
    pub fn dequeue(&mut self) -> Option<T> {
        if let Some(mut head) = self.head.take(){
            self.length -= 1;
            if let Some(next) = head.next.take(){
                self.head = Some(next);
            } else {
                self.tail = None;
            }
            Some(head.value)
        } else {
            None }
    }

    pub fn peek(&self) -> usize {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn length(&self) -> usize {
        self.length
    }
}



