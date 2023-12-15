use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);

    let elapsed_time = now.elapsed();
    print_elapsed_time(elapsed_time);
}

fn print_elapsed_time(elapsed_time: std::time::Duration) {
    if elapsed_time.as_secs() == 0 {
        println!(
            "solving the problem took {} milliseconds.",
            elapsed_time.as_millis()
        );
    } else {
        println!(
            "solving the problem took {} seconds.",
            elapsed_time.as_secs()
        );
    }
}

fn part1(input: &str) -> usize {
    let tilted = tilt_north(input);
    get_load(tilted)
}

fn tilt_north(input: &str) -> Vec<Vec<char>> {
    let cols = input.lines().next().unwrap().len();

    let mut tilted: Vec<Vec<char>> = vec![vec!['.'; cols]; cols];
    let mut boulders: Vec<Vec<usize>> = vec![vec![]; cols];
    let mut cubes: Vec<Vec<usize>> = vec![vec![]; cols];

    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                'O' => boulders[col].push(row),
                '#' => {
                    tilted[row][col] = '#';
                    cubes[col].push(row);
                }
                '.' => {}
                _ => panic!("unexpected char: {}", c),
            }
        }
    }

    for (col, boulder_rows) in boulders.into_iter().enumerate() {
        for row in boulder_rows.into_iter() {
            let mut end_row = row;
            for i in 1..=row {
                if tilted[row - i][col] != '.' {
                    break;
                }
                end_row = row - i;
            }
            tilted[end_row][col] = 'O';
        }
    }

    tilted
}

fn get_load(tilted: Vec<Vec<char>>) -> usize {
    let mut load = 0;
    let rows = tilted.len();
    let cols = tilted[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if tilted[row][col] == 'O' {
                load += rows - row;
            }
        }
    }

    load
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        assert_eq!(part1(input), 136);
    }

    #[test]
    fn test_tilt_north() {
        let input: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

        let expected = r#"OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."#;

        let result = tilt_north(input)
            .into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        println!("{}", result);
        assert_eq!(result, expected);
    }
}
