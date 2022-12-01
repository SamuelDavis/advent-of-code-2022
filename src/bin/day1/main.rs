use std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/day1/input.txt").expect("read input");
    let inventory = input.split("\n\n").into_iter().map(|inventory| {
        inventory
            .split("\n")
            .into_iter()
            .map(|calories| calories.parse::<i32>().unwrap_or(0))
            .sum::<i32>()
    });
    let maximum = inventory.max().expect("numeric maximum");
    println!("{:#?}", maximum);
}
