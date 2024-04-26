type NodePtr<T> = Option<Box<Node<T>>>;

pub struct Stack<T: Clone> {
    head: NodePtr<T>,
    size: isize,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None, size: 0, }
    }

    pub fn push(&mut self, item: T) {
        let prev: NodePtr<T> = self.head.clone();
        self.head = Some(Box::new(Node::new(item, prev)));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(popped) => {
                self.head = popped.get_next();
                self.size -= 1;
                Some(popped.get_item())
            }
        }
    }

    pub fn len(&self) -> isize { self.size }
}

#[derive(Clone)]
struct Node<T: Clone> {
    item: T,
    next: NodePtr<T>,
}

impl<T: Clone> Node<T> {
    fn new(item: T, next: NodePtr<T>) -> Self {
        Node { item, next }
    }

    fn get_item(&self) -> T          { self.item.clone() }
    fn get_next(&self) -> NodePtr<T> { self.next.clone() }
}
