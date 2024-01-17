#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Stack<T> {
    length: usize,
    head: Option<Box<Node<T>>>,
    tail: Optioni<Box<Node<T>>>,
}
