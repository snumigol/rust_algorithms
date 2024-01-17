#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Stack<T> {
     top: Option<Box<Node<T>>>,
}

