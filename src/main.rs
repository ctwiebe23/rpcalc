use rpcalc::Stack;

fn main() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(32);
    stack.push(42);
    stack.push(10);
    stack.push(3);
    println!("{:?}, size = {:?}", stack.pop().unwrap_or(0), stack.len());
    println!("{:?}, size = {:?}", stack.pop().unwrap_or(0), stack.len());
    println!("{:?}, size = {:?}", stack.pop().unwrap_or(0), stack.len());
    println!("{:?}, size = {:?}", stack.pop().unwrap_or(0), stack.len());
    println!("{:?}, size = {:?}", stack.pop().unwrap_or(0), stack.len());
    println!("{:?}, size = {:?}", stack.pop().unwrap_or(0), stack.len());
    println!("{:?}, size = {:?}", stack.pop().unwrap_or(0), stack.len());
    println!("{:?}, size = {:?}", stack.pop().unwrap_or(0), stack.len());
}
