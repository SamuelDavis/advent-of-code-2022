#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

fn main() {
    let guide = std::fs::read_to_string("./src/bin/day02/input.txt").expect("contents");
    let score = calculate_score_from_tournament(guide);
    dbg!(score);
}

fn calculate_score_from_tournament(input: String) -> i32 {
    input
        .lines()
        .map(|line| {
            let (theirs, yours) = line.split_once(' ').expect("input and output");
            let theirs = get_shape_from_theirs(theirs).expect("theirs");
            let yours = get_shape_from_yours(yours).expect("yours");
            let outcome = get_your_outcome_from_theirs_and_yours(&theirs, &yours);

            get_score_from_shape(&yours) + get_score_from_outcome(&outcome)
        })
        .sum()
}

fn get_score_from_outcome(outcome: &Outcome) -> i32 {
    match outcome {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn get_score_from_shape(shape: &Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn get_your_outcome_from_theirs_and_yours(theirs: &Shape, yours: &Shape) -> Outcome {
    if theirs == yours {
        return Outcome::Draw;
    }
    match theirs {
        Shape::Rock => match yours {
            Shape::Rock => Outcome::Draw,
            Shape::Paper => Outcome::Win,
            Shape::Scissors => Outcome::Lose,
        },
        Shape::Paper => match yours {
            Shape::Rock => Outcome::Lose,
            Shape::Paper => Outcome::Draw,
            Shape::Scissors => Outcome::Win,
        },
        Shape::Scissors => match yours {
            Shape::Rock => Outcome::Win,
            Shape::Paper => Outcome::Lose,
            Shape::Scissors => Outcome::Draw,
        },
    }
}

fn get_shape_from_theirs(value: &str) -> Option<Shape> {
    match value {
        "A" => Some(Shape::Rock),
        "B" => Some(Shape::Paper),
        "C" => Some(Shape::Scissors),
        _ => None,
    }
}

fn get_shape_from_yours(value: &str) -> Option<Shape> {
    match value {
        "X" => Some(Shape::Rock),
        "Y" => Some(Shape::Paper),
        "Z" => Some(Shape::Scissors),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_shape_from_theirs() {
        let data = vec![
            ("A", Some(Shape::Rock)),
            ("B", Some(Shape::Paper)),
            ("C", Some(Shape::Scissors)),
            ("D", None),
        ];

        for (input, expected) in data {
            assert_eq!(expected, get_shape_from_theirs(input));
        }
    }

    #[test]
    fn test_get_shape_from_yours() {
        let data = vec![
            ("X", Some(Shape::Rock)),
            ("Y", Some(Shape::Paper)),
            ("Z", Some(Shape::Scissors)),
            ("W", None),
        ];

        for (input, expected) in data {
            assert_eq!(expected, get_shape_from_yours(input));
        }
    }

    #[test]
    fn test_get_outcome_from_theirs_and_yours() {
        let data = vec![
            (Shape::Rock, Shape::Rock, Outcome::Draw),
            (Shape::Rock, Shape::Paper, Outcome::Win),
            (Shape::Rock, Shape::Scissors, Outcome::Lose),
            (Shape::Paper, Shape::Rock, Outcome::Lose),
            (Shape::Paper, Shape::Paper, Outcome::Draw),
            (Shape::Paper, Shape::Scissors, Outcome::Win),
            (Shape::Scissors, Shape::Rock, Outcome::Win),
            (Shape::Scissors, Shape::Paper, Outcome::Lose),
            (Shape::Scissors, Shape::Scissors, Outcome::Draw),
        ];

        for (theirs, yours, expected) in data {
            assert_eq!(
                expected,
                get_your_outcome_from_theirs_and_yours(&theirs, &yours)
            );
        }
    }

    #[test]
    fn test_calculate_score_from_tournament() {
        let input = "A Y\nB X\nC Z".to_string();
        let expected = 15;
        assert_eq!(expected, calculate_score_from_tournament(input))
    }
}
