use std::isize;

fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);
}

#[derive(Debug)]
struct SchematicNumber<'a> {
    characters: &'a str,
    position: (usize, usize),
}

impl SchematicNumber<'_> {
    fn is_adjacent_to(&self, y: usize, x: usize) -> bool {
        let start_y = self.position.0 as isize;
        let start_x = self.position.1 as isize;
        let num_len = self.characters.len() as isize;

        let bounds_y = (start_y - 1)..(start_y + 2);
        let bounds_x = (start_x - 1)..(start_x + num_len + 1);
        return bounds_y.contains(&(y as isize)) && bounds_x.contains(&(x as isize));
    }

    fn to_number(&self) -> usize {
        self.characters.parse::<usize>().unwrap()
    }
}

#[derive(Debug)]
struct Gear<'a> {
    _position: (usize, usize),
    adjacent_numbers: Vec<&'a SchematicNumber<'a>>,
}

fn part2(input: &str) -> usize {
    let schematic_numbers = get_schematic_numbers(input);

    let gears = get_gears(&schematic_numbers, input);

    gears
        .iter()
        .filter(|g| g.adjacent_numbers.len() == 2)
        .map(|g| {
            let mut adjacent_numbers = g.adjacent_numbers.iter();
            let first = adjacent_numbers.next().unwrap();
            let second = adjacent_numbers.next().unwrap();
            first.to_number() * second.to_number()
        })
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

fn get_gears<'a>(numbers: &'a Vec<SchematicNumber<'a>>, input: &str) -> Vec<Gear<'a>> {
    let mut gears: Vec<Gear<'a>> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '*' {
                let mut gear = Gear {
                    _position: (y, x),
                    adjacent_numbers: Vec::new(),
                };
                for number in numbers {
                    if number.is_adjacent_to(y, x) {
                        gear.adjacent_numbers.push(&number);
                    }
                }
                gears.push(gear);
            }
        }
    }
    gears
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
.664.598..
"#;
        assert_eq!(super::part2(input), 467835);
    }
}
