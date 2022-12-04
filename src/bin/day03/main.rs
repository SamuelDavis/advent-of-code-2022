use std::collections::HashSet;

type Item = char;
type Priority = i32;

fn main() {
    let input = std::fs::read_to_string("./src/bin/day03/input.txt").expect("input");
    println!("{}", calculate_sum_of_inventory_priorities(&input));
}

fn calculate_sum_of_inventory_priorities(input: &String) -> i32 {
    let inventories = input.lines();

    inventories
        .to_owned()
        .map(|line| line.chars().collect())
        .collect::<Vec<HashSet<Item>>>()
        .chunks(3)
        .map(|group| {
            let a = group[0].to_owned();
            let b = group[1].to_owned();
            let c = group[2].to_owned();
            let y = derive_common_items(a.to_owned(), b.to_owned());
            let z = derive_common_items(a.to_owned(), c.to_owned());

            derive_common_items(y, z)
                .into_iter()
                .next()
                .expect("one result")
        })
        .map(prioritize_item)
        .sum()
}

fn derive_common_items(set_a: HashSet<Item>, set_b: HashSet<Item>) -> HashSet<Item> {
    let mut common_items = HashSet::new();
    for item in set_a.intersection(&set_b).into_iter() {
        common_items.insert(item.to_owned());
    }

    common_items
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
        assert_eq!(70, calculate_sum_of_inventory_priorities(&input));
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
    fn test_prioritize_item() {
        let data = vec![('a', 1), ('z', 26), ('A', 27), ('Z', 52)];

        for (input, expected) in data {
            assert_eq!(expected, prioritize_item(input));
        }
    }
}
