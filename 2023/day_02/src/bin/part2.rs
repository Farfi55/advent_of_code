fn main() {
    let input = include_str!("./input2.txt");
    let result = part2(input);
    println!("sum:\n{}", result);
}

fn handle_line(line: &str) -> u32 {
    let game_parts = line.split(':');
    let cubes_reveals = game_parts.skip(1).next().unwrap().split(';');

    let mut min_cubes = [0; 3];

    for cubes_reveal in cubes_reveals {
        let color_cubes = cubes_reveal.split(',');

        for color_cube in color_cubes {
            let mut count_color = color_cube.trim().split(' ');
            let count = count_color.next().unwrap().parse::<u32>().unwrap();
            let color = count_color.next().unwrap();

            let color_idx = match color {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => panic!("unknown color"),
            };
            if count > min_cubes[color_idx] {
                min_cubes[color_idx] = count;
            }
        }
    }
    min_cubes[0] * min_cubes[1] * min_cubes[2]
}

fn part2(input: &str) -> u32 {
    input.lines().map(|line| handle_line(line)).sum()
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

        let result = super::part2(input);
        assert_eq!(result, 2286);
    }
}
