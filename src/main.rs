use rpcalc::solve;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut expression: Vec<String> = Vec::new();

    for a in args {
        expression.append(&mut a.split(" ")
            .map(|s| s.to_string())
            .collect());
    }

    match solve(expression) {
        Ok(ok) => print!("{ok}\n"),
        Err(e) => print!("{e}\n"),
    }
}
