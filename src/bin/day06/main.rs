use std::collections::HashSet;
use std::hash::Hash;

const BUFFER_LENGTH: usize = 14;

fn main() {
    let input = std::fs::read_to_string("./src/bin/day06/input.txt").expect("input");
    println!("{}", find_unique_string_index(input).expect("output"));
}

fn find_unique_string_index(input: String) -> Option<usize> {
    let mut buffer = [' '; BUFFER_LENGTH];
    for (i, c) in input.char_indices() {
        let j = i % BUFFER_LENGTH;
        buffer[j] = c;
        if i >= BUFFER_LENGTH && is_unique(buffer.to_owned()) {
            return Some(i + 1);
        }
    }
    return None;
}

fn is_unique<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input() {
        let input = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(19)),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(23)),
            ("nppdvjthqldpwncqszvftbrmjlhg", Some(23)),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Some(29)),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Some(26)),
        ];
        for (input, expected) in input {
            assert_eq!(expected, find_unique_string_index(input.to_string()))
        }
    }

    #[test]
    fn test_is_unique() {
        let input = vec![
            (['a', 'b', 'c', 'd'], true),
            (['a', 'b', 'c', 'a'], false),
            (['z', 'b', 'c', 'd'], true),
            (['z', 'b', 'c', 'z'], false),
        ];
        for (input, expected) in input {
            assert_eq!(expected, is_unique(input));
        }
    }
}
