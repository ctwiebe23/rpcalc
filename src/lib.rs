type NodePtr<T> = Option<Box<Node<T>>>;

// simple implementation of the standard stack data structure
//
// features push, pop, and len
//
// cannot be constructed from an existing collection
struct Stack<T: Clone> {
    head: NodePtr<T>,
    size: isize,
}

impl<T: Clone> Stack<T> {
    fn new() -> Self {
        Stack { head: None, size: 0, }
    }

    fn push(&mut self, item: T) {
        let prev: NodePtr<T> = self.head.clone();
        self.head = Some(Box::new(Node::new(item, prev)));
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(popped) => {
                self.head = popped.get_next();
                self.size -= 1;
                Some(popped.get_item())
            }
        }
    }

    fn len(&self) -> isize { self.size }
}

// individual nodes in the stack, constructed in the style of a linked list
#[derive(Clone)]
struct Node<T: Clone> {
    item: T,
    next: NodePtr<T>,
}

impl<T: Clone> Node<T> {
    fn new(item: T, next: NodePtr<T>) -> Self {
        Node { item, next }
    }

    fn get_item(&self) -> T { self.item.clone() }
    fn get_next(&self) -> NodePtr<T> { self.next.clone() }
}

// evaluates the expression formed by the operator and 2 elements
fn evaluate(a: f64, b: f64, operator: String) -> f64 {
    match operator.as_str() {
        "+" => a + b,
        "-" => a - b,
        "*" | "x" => a * b,
        "/" => a / b,
        "%" => a % b,
        "^" => a.powf(b),
        "log" => a.log(b),
        _ => panic!("{} is not an operator", operator),
    }
}

// solves the expression represented by a vector of strings in reverse polish
// notation
pub fn solve(expression: Vec<String>) -> f64 {
    let mut cache: Stack<f64> = Stack::new();
    for o in expression {
        match o.parse::<f64>() {
            Ok(ok) => cache.push(ok),
            Err(_) if o == "e" => cache.push(std::f64::consts::E),
            Err(_) if o == "pi" => cache.push(std::f64::consts::PI),
            Err(_) => {
                if cache.len() < 2 { panic!("too few arguments for {}", o); }
                let (b, a) = (cache.pop().unwrap(), cache.pop().unwrap());
                cache.push(evaluate(a, b, o));
            }
        }
    }
    if cache.len() != 1 { panic!("values left unused after evaluation"); }
    cache.pop().unwrap()
}
