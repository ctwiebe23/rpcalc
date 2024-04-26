use rpcalc::solve;

fn main() {
    let expression: Vec<String> = std::env::args().skip(1).collect();
    print!("{}\n", solve(expression));
}
