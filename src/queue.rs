#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}


#[derive(Debug)]
pub struct Queue<T> {
    length: usize,
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
}
