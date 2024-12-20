mod solve;

fn main() {
    let raw_input = std::fs::read_to_string("./src/input.txt").unwrap();
    println!("{}", solve::solve(&raw_input));
}
