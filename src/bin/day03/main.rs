use std::collections::HashSet;

type Item = char;
type Priority = i32;

fn main() {
    let input = std::fs::read_to_string("./src/bin/day03/input.txt").expect("input");
    println!("{}", calculate_sum_of_inventory_priorities(&input));
}

fn calculate_sum_of_inventory_priorities(input: &String) -> i32 {
    input
        .lines()
        .map(|line| {
            let (compartment_a, compartment_b) = derive_compartments(line.chars().collect());
            derive_common_items(compartment_a, compartment_b)
        })
        .flatten()
        .map(prioritize_item)
        .sum::<Priority>()
}

fn derive_common_items(
    compartment_a: HashSet<Item>,
    compartment_b: HashSet<Item>,
) -> HashSet<Item> {
    let mut common_items = HashSet::new();
    for item in compartment_a.intersection(&compartment_b).into_iter() {
        common_items.insert(item.to_owned());
    }

    common_items
}

fn derive_compartments(items: Vec<Item>) -> (HashSet<Item>, HashSet<Item>) {
    let items = items.into_iter();
    let compartment_size = items.len() / 2;
    let compartment_a: HashSet<Item> = items.to_owned().take(compartment_size).collect();
    let compartment_b: HashSet<Item> = items.to_owned().skip(compartment_size).collect();

    (compartment_a, compartment_b)
}

fn prioritize_item(item: Item) -> Priority {
    if !item.is_alphabetic() {
        panic!("can only handle alphabetic items")
    }

    let augend = if item.is_lowercase() { 1 } else { 27 };
    let subtrahend = if item.is_lowercase() { 'a' } else { 'A' } as i32;

    augend + item as i32 - subtrahend
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        let input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
        .join("\n");
        assert_eq!(157, calculate_sum_of_inventory_priorities(&input));
    }

    #[test]
    fn test_derive_common_items() {
        assert_eq!(
            HashSet::from(['p']),
            derive_common_items(
                HashSet::from(['v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r']),
                HashSet::from(['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p']),
            )
        )
    }

    #[test]
    fn test_derive_compartments() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect();
        let expected = (
            HashSet::from(['v', 'J', 'r', 'w', 'p', 'W', 't', 'w', 'J', 'g', 'W', 'r']),
            HashSet::from(['h', 'c', 's', 'F', 'M', 'M', 'f', 'F', 'F', 'h', 'F', 'p']),
        );
        assert_eq!(expected, derive_compartments(input));
    }

    #[test]
    fn test_prioritize_item() {
        let data = vec![('a', 1), ('z', 26), ('A', 27), ('Z', 52)];

        for (input, expected) in data {
            assert_eq!(expected, prioritize_item(input));
        }
    }
}
