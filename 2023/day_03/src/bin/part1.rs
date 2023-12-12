use std::isize;

fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

#[derive(Debug)]
struct SchematicNumber<'a> {
    characters: &'a str,
    position: (usize, usize),
}

fn part1(input: &str) -> usize {
    let schematic_numbers = get_schematic_numbers(input);

    let filtered_schematic_numbers = filter_schematic_numbers(schematic_numbers, input);

    filtered_schematic_numbers
        .iter()
        .map(|n| n.characters.parse::<usize>().unwrap())
        .sum()
}

fn get_schematic_numbers(input: &str) -> Vec<SchematicNumber> {
    let mut schematic_numbers = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut num_start_x: Option<usize> = None;
        for (x, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if num_start_x.is_none() {
                    num_start_x = Some(x);
                }
            } else {
                if num_start_x.is_some() {
                    let start_x = num_start_x.unwrap();
                    let characters = &line[start_x..x];
                    schematic_numbers.push(SchematicNumber {
                        characters,
                        position: (y, start_x),
                    });
                    num_start_x = None;
                }
            }
        }

        if num_start_x.is_some() {
            let start_x = num_start_x.unwrap();
            let characters = &line[start_x..];
            schematic_numbers.push(SchematicNumber {
                characters,
                position: (y, start_x),
            })
        }
    }
    schematic_numbers
}

fn filter_schematic_numbers<'a>(
    numbers: Vec<SchematicNumber<'a>>,
    input: &str,
) -> Vec<SchematicNumber<'a>> {
    numbers
        .into_iter()
        .filter(|number| has_symbol_adjacent(&number, input))
        .collect()
}
// 536649
fn has_symbol_adjacent(number: &SchematicNumber, input: &str) -> bool {
    let (start_y, start_x) = number.position;
    let num_len = number.characters.len();
    for y in (start_y as isize - 1)..(start_y as isize + 2) {
        for x in (start_x as isize - 1)..(start_x as isize + num_len as isize + 1) {
            let x = x as usize;
            let y = y as usize;
            if y == start_y && x >= start_x && x < start_x + num_len {
                continue;
            }
            if is_symbol(y, x, input) {
                return true;
            }
        }
    }
    false
}

fn is_symbol(y: usize, x: usize, input: &str) -> bool {
    let mut lines = input.lines();
    let line = match lines.nth(y) {
        Some(line) => line,
        None => return false,
    };
    let c: char = match line.chars().nth(x) {
        Some(c) => c,
        None => return false,
    };
    if c.is_numeric() || c == '.' {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let result = super::part1(input);
        assert_eq!(result, 4361);
    }

    #[test]
    fn is_symbol_works() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        assert_eq!(super::is_symbol(1, 0, input), false);
        assert_eq!(super::is_symbol(1, 3, input), true);
        assert_eq!(super::is_symbol(2, 2, input), false);
        assert_eq!(super::is_symbol(3, 7, input), false);
    }

    #[test]
    fn test_2() {
        let input = r#"..........
..........
....123...
.........."#;
        assert_eq!(super::part1(input), 0);
    }

    #[test]
    fn test_3() {
        let input = r#"..........
...*......
....123...
.........."#;
        assert_eq!(super::part1(input), 123);
    }

    #[test]
    fn test_4() {
        let input = r#"**********
***.....**
***.123.**
***.....**
**********"#;
        assert_eq!(super::part1(input), 0);
    }
}
