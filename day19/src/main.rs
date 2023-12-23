mod parser;
mod part1;
mod part2;
mod tokenizer;

fn main() {
    let input = "./src/input.txt";
    println!("{}", part1::run(input));
    println!("{}", part2::run(input));
}
