use std::fs;

fn main() {
    let input = fs::read_to_string("./src/bin/day01/input.txt").expect("read input");
    let mut inventory: Vec<i32> = input
        .split("\n\n")
        .into_iter()
        .map(|inventory| {
            inventory
                .split("\n")
                .into_iter()
                .map(|calories| calories.parse::<i32>().unwrap_or(0))
                .sum()
        })
        .collect();
    inventory.sort();
    let top_three: i32 = inventory.iter().rev().take(3).sum();
    println!("{:#?}", top_three);
}
