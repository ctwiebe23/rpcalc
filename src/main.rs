use rpcalc::solve;

fn main() {
    let expression: Vec<String> = std::env::args()
        .collect::<Vec<String>>()[1]
        .split(" ")
        .map(|s| s.to_string())
        .collect();

    print!("{}\n", solve(expression));
}
