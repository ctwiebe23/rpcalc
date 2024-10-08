// mozzie
//
// contains both a stack and a postfix calculator

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
        Stack { head: None, size: 0 }
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

// rounds the given number n to the given number of decimal points
fn rnd(n: f64, precision: f64) -> f64 {
    let exp: f64 = 10_f64.powf(precision.round());

    (n * exp).round() / exp
}

// takes the factorial of n, returning an error if n is negative
fn factorial(n: f64) -> Result<f64, String> {
    if n < 0.0 {
        return Err(format!("cannot take factorial of negative number {n}"));
    }

    let mut result: f64 = 1.0;
    let limit: u32 = (n as u32) + 1;

    for i in 1..limit {
        result *= i as f64;
    }

    Ok(result)
}

// evaluates the expression formed by the operator and 2 elements
fn evaluate(cache: &mut Stack<f64>, operator: String) -> Result<f64, String> {
    let safe_pop = |stack: &mut Stack<f64>| match stack.pop() {
        None => Err(format!("too few arguments for {operator}")),
        Some(x) => Ok(x),
    };

    let b: f64 = safe_pop(cache)?;

    match operator.as_str() {
        "+"         => Ok(safe_pop(cache)? + b),
        "-"         => Ok(safe_pop(cache)? - b),
        "*" | "x"   => Ok(safe_pop(cache)? * b),
        "/"         => Ok(safe_pop(cache)? / b),
        "%"         => Ok(safe_pop(cache)? % b),
        "^"         => Ok(safe_pop(cache)?.powf(b)),
        "log"       => Ok(safe_pop(cache)?.log(b)),
        "rnd"       => Ok(rnd(safe_pop(cache)?, b)),
        "ln"        => Ok(b.ln()),
        "log2"      => Ok(b.log2()),
        "log10"     => Ok(b.log10()),
        "sin"       => Ok(b.sin()),
        "cos"       => Ok(b.cos()),
        "tan"       => Ok(b.tan()),
        "csc"       => Ok(1.0 / b.sin()),
        "sec"       => Ok(1.0 / b.cos()),
        "cot"       => Ok(1.0 / b.tan()),
        "arcsin"    => Ok(b.asin()),
        "arccos"    => Ok(b.acos()),
        "arctan"    => Ok(b.atan()),
        "arccsc"    => Ok(1.0 / b.asin()),
        "arcsec"    => Ok(1.0 / b.acos()),
        "arccot"    => Ok(1.0 / b.atan()),
        "!"         => factorial(b.round()),
        _           => Err(format!("{operator} is not an operator")),
    }
}

// solves the expression represented by a vector of strings in reverse polish
// notation
pub fn solve(expression: Vec<String>) -> Result<f64, String> {
    let mut cache: Stack<f64> = Stack::new();

    for v in expression {
        match v.parse::<f64>() {
            Ok(ok)              => cache.push(ok),
            Err(_) if v == "e"  => cache.push(std::f64::consts::E),
            Err(_) if v == "G"  => cache.push(6.6743E-11),
            Err(_) if v == "g"  => cache.push(9.81),
            Err(_) if v == "c"  => cache.push(299792458.0),
            Err(_) if v == "pi" => cache.push(std::f64::consts::PI),
            Err(_) => {
                let result: f64 = evaluate(&mut cache, v)?;
                cache.push(result)
            }
        }
    }

    match cache.len() {
        1 => Ok(cache.pop().unwrap()),
        0 => Err("cache empty; no values given".to_string()),
        _ => Err("invalid syntax; values left unused".to_string()),
    }
}
