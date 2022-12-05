fn main() {
    let input = std::fs::read_to_string("./src/bin/day04/input.txt").expect("input");
    println!("{}", find_partially_overlapping_ranges(input));
}

#[derive(Debug)]
pub struct SectionAssignment {
    lower: u32,
    upper: u32,
}

impl SectionAssignment {
    pub fn new(lower: u32, upper: u32) -> Self {
        Self { lower, upper }
    }

    pub fn from_str(value: &str) -> Self {
        let (lower, upper) = value.split_once("-").expect("range described by hyphen");
        SectionAssignment::new(
            lower.parse().expect("numeric"),
            upper.parse().expect("numeric"),
        )
    }

    pub fn overlaps(&self, other: &SectionAssignment) -> bool {
        self.upper >= other.lower && self.lower <= other.upper
    }
}

impl PartialEq<Self> for SectionAssignment {
    fn eq(&self, other: &Self) -> bool {
        self.lower == other.lower && self.upper == other.upper
    }
}

fn find_partially_overlapping_ranges(input: String) -> i32 {
    input.lines().fold(0, |acc, line| {
        let (a, b) = line.split_once(",").expect("csv");
        let a = SectionAssignment::from_str(a);
        let b = SectionAssignment::from_str(b);
        acc + (a.overlaps(&b) || b.overlaps(&a)) as i32
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        let input = vec![
            "2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8",
        ]
        .join("\n");
        assert_eq!(4, find_partially_overlapping_ranges(input));
    }

    #[test]
    fn test_section_assignment_from_str() {
        let input = vec![
            ("0-0", SectionAssignment::new(0, 0)),
            ("0-1", SectionAssignment::new(0, 1)),
            ("9-10", SectionAssignment::new(9, 10)),
            ("99-1000", SectionAssignment::new(99, 1000)),
        ];

        for (input, expected) in input {
            assert_eq!(expected, SectionAssignment::from_str(input));
        }
    }
}
