fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);
}

fn part2(_input: &str) -> usize {
    let instructions: Vec<Instruction> = _input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|i| Instruction::from_str(i))
        .collect();

    let mut boxes: Vec<Vec<&Instruction>> = vec![vec![]; 256];
    for (i, instruction) in instructions.iter().enumerate() {
        boxes = apply_instruction(boxes, &instruction);

        if i > 0 {
            println!()
        }
        println!("After \"{}\":", instruction.to_raw_str());
        print_boxes(&boxes);
    }

    boxes
        .iter()
        .map(|lens_box| get_focusing_power(lens_box))
        .sum()
}

fn print_boxes(boxes: &Vec<Vec<&Instruction>>) -> () {
    for (i, b) in boxes.iter().enumerate() {
        if b.is_empty() {
            continue;
        }
        let boxes_str = b
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("Box {}: {}", i, boxes_str);
    }
}

fn apply_instruction<'a>(
    mut boxes: Vec<Vec<&'a Instruction<'a>>>,
    instruction: &'a Instruction,
) -> Vec<Vec<&'a Instruction<'a>>> {
    match instruction {
        Instruction::Set(label, box_hash, lense) => {
            let lens_box = &mut boxes[*box_hash];
            let index = lens_box.iter().position(|x| match x {
                Instruction::Set(l, _, _) => l == label,
                Instruction::Del(l, _) => l == label,
            });

            match index {
                Some(i) => lens_box[i] = instruction,
                None => lens_box.push(&instruction),
            }
        }
        Instruction::Del(label, box_hash) => {
            let lens_box = &mut boxes[*box_hash];
            lens_box.retain(|i| match i {
                Instruction::Set(l, _, _) => l != label,
                Instruction::Del(l, _) => l != label,
            });
        }
    }
    boxes
}

fn get_focusing_power(lens_box: &[&Instruction<'_>]) -> usize {
    let mut power_sum = 0;
    for (i, instruction) in lens_box.iter().enumerate() {
        match instruction {
            Instruction::Set(_, box_hash, lense) => power_sum += (box_hash + 1) * (i + 1) * lense,
            Instruction::Del(_, _) => panic!("Cannot focus on a deleted box"),
        }
    }

    power_sum
}

fn hash_instruction(instruction: &str) -> usize {
    instruction.chars().fold(0, |acc, c| hash(c, acc))
}

fn hash(c: char, n: usize) -> usize {
    ((n + c as usize) * 17) % 256
}

#[derive(Debug, PartialEq)]
enum Instruction<'a> {
    Set(&'a str, usize, usize),
    Del(&'a str, usize),
}

impl Instruction<'_> {
    fn from_str(instruction: &str) -> Instruction {
        if instruction.chars().last().unwrap() == '-' {
            let label = instruction.trim_end_matches('-');
            let box_hash = hash_instruction(label);

            Instruction::Del(label, box_hash)
        } else {
            let parts: Vec<&str> = instruction.split('=').collect();
            let label = parts[0];
            let hash_value = hash_instruction(label);
            let lense = parts[1].parse::<usize>().unwrap();

            Instruction::Set(label, hash_value, lense)
        }
    }

    fn to_raw_str(&self) -> String {
        match self {
            Instruction::Set(label, _, lense) => format!("{}={}", label, lense),
            Instruction::Del(label, _) => format!("{}-", label),
        }
    }
}

impl std::fmt::Display for Instruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Set(label, _, lense) => {
                write!(f, "[{} {}]", label, lense)
            }
            Instruction::Del(label, hash_value) => write!(f, "[del {}-{}]", label, hash_value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part2(input), 145);
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

    #[test]
    fn test_instruction_from_str() {
        let input = vec![
            ("rn=1", Instruction::Set("rn", 0, 1)),
            ("cm-", Instruction::Del("cm", 0)),
            ("qp=3", Instruction::Set("qp", 1, 3)),
            ("cm=2", Instruction::Set("cm", 0, 2)),
            ("qp-", Instruction::Del("qp", 1)),
            ("pc=4", Instruction::Set("pc", 3, 4)),
            ("ot=9", Instruction::Set("ot", 3, 9)),
            ("ab=5", Instruction::Set("ab", 3, 5)),
            ("pc-", Instruction::Del("pc", 3)),
            ("pc=6", Instruction::Set("pc", 3, 6)),
            ("ot=7", Instruction::Set("ot", 3, 7)),
        ];

        for (instruction, expected) in input {
            assert_eq!(Instruction::from_str(instruction), expected);
        }
    }
}
