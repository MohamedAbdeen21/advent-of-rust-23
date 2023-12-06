mod part1;
mod part2;

fn main() {
    let filename = "./src/input.txt";
    println!("{}", part1::run(filename));
    println!("{}", part2::run(filename));
}
