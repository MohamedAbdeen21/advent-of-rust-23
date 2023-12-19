mod parser;
mod part1;
mod tokenizer;

fn main() {
    let input = "./src/input.txt";
    println!("{}", part1::run(input));
}
