fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(_input: &str) -> usize {
    let instructions: Vec<&str> = _input.lines().next().unwrap().split(',').collect();
    let hash_instructions: Vec<usize> = instructions.iter().map(|i| hash_instruction(i)).collect();
    hash_instructions.iter().sum()
}

fn hash_instruction(instruction: &str) -> usize {
    instruction.chars().fold(0, |acc, c| hash(c, acc))
}

fn hash(c: char, n: usize) -> usize {
    ((n + c as usize) * 17) % 256
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(input), 1320);
    }

    #[test]
    fn hash_instructions_correctly() {
        let input = vec![
            ("rn=1", 30),
            ("cm-", 253),
            ("qp=3", 97),
            ("cm=2", 47),
            ("qp-", 14),
            ("pc=4", 180),
            ("ot=9", 9),
            ("ab=5", 197),
            ("pc-", 48),
            ("pc=6", 214),
            ("ot=7", 231),
        ];

        for (instruction, expected) in input {
            assert_eq!(hash_instruction(instruction), expected);
        }
    }
}
