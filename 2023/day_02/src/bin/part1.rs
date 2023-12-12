fn main() {
    let input = include_str!("./input1.txt");
    let result = part1(input);
    println!("sum:\n{}", result);
}

fn handle_line(line: &str, max_red: u32, max_green: u32, max_blue: u32) -> u32 {
    let mut game_parts = line.split(':');
    let id = game_parts.next().unwrap()[5..].parse::<u32>().unwrap();
    let game_body = game_parts.next().unwrap();
    let cubes_reveals = game_body.split(';');

    for cubes_reveal in cubes_reveals {
        let color_cubes = cubes_reveal.split(',');

        for color_cube in color_cubes {
            let mut count_color = color_cube.trim().split(' ');
            let count = count_color.next().unwrap().parse::<u32>().unwrap();
            let color = count_color.next().unwrap();

            match color {
                "red" => {
                    if count > max_red {
                        return 0;
                    }
                }
                "green" => {
                    if count > max_green {
                        return 0;
                    }
                }
                "blue" => {
                    if count > max_blue {
                        return 0;
                    }
                }
                _ => panic!("unknown color"),
            }
        }
    }

    id
}

fn part1(input: &str) -> u32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    input
        .lines()
        .map(|line| handle_line(line, max_red, max_green, max_blue))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

        let result = super::part1(input);
        assert_eq!(result, 8);
    }
}
