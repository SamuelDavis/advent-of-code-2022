fn main() {
    let input = std::fs::read_to_string("./src/bin/day05/input.txt").expect("input");
    println!("{}", process(input));
}

type Supplies = char;
type Stack = Vec<Supplies>;
type Cargo = Vec<Stack>;
type Instruction = (usize, usize, usize);
type Procedure = Vec<Instruction>;

fn process(input: String) -> String {
    let (drawing, procedure) = input.split_once("\n\n").expect("input");
    let cargo = make_cargo_from_string(drawing.to_string());
    let procedures = make_procedure_from_string(procedure.to_string());
    let result = execute_procedure(cargo, procedures);
    get_top_from_cargo(result)
}

fn get_top_from_cargo(cargo: Cargo) -> String {
    cargo.into_iter().fold(String::new(), |mut acc, c| {
        let c: char = *c.last().unwrap_or(&' ');
        acc.push(c);
        acc
    })
}

fn make_cargo_from_string(input: String) -> Cargo {
    let mut lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let length = lines.pop().expect("count").len();
    lines.reverse();
    let mut cargo: Cargo = Vec::new();
    for i in (1..length).step_by(4) {
        let mut stack: Stack = Vec::new();
        for line in &lines {
            let supplies: Supplies = line[i];
            if supplies != ' ' {
                stack.push(supplies);
            }
        }
        cargo.push(stack);
    }
    cargo
}

fn make_procedure_from_string(input: String) -> Procedure {
    input
        .lines()
        .map(|line| {
            let [count, from, to]: [usize; 3] = line
                .split(" ")
                .map(|v| v.to_string().parse::<usize>())
                .filter(|v| v.is_ok())
                .map(|v| v.unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .expect("3 numerics");
            (count, from, to)
        })
        .collect()
}

fn execute_procedure(mut cargo: Cargo, procedure: Procedure) -> Cargo {
    for (count, from, to) in procedure {
        let final_length = cargo[from - 1].len() - count;
        let mut tail = cargo[from - 1].split_off(final_length);
        cargo[to - 1].append(&mut tail);
    }
    cargo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_works() {
        let input = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]
        .join("\n");
        assert_eq!("MCD", process(input));
    }

    #[test]
    fn test_make_cargo_from_template() {
        let input = vec!["    [D]    ", "[N] [C]    ", "[Z] [M] [P]", " 1   2   3 "]
            .join("\n")
            .to_string();
        let expected: Cargo = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(expected, make_cargo_from_string(input));
    }

    #[test]
    fn test_make_procedure_from_string() {
        let input = vec![
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]
        .join("\n")
        .to_string();
        let expected: Procedure = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];
        assert_eq!(expected, make_procedure_from_string(input));
    }

    #[test]
    fn test_execute_procedure() {
        let cargo: Cargo = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let procedure: Procedure = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];
        let expected: Cargo = vec![vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']];
        assert_eq!(expected, execute_procedure(cargo, procedure));
    }
}
