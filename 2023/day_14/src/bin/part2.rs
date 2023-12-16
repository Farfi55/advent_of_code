use std::{collections::HashMap, time::Instant};

fn main() {
    let now = Instant::now();

    let input = include_str!("./input.txt");
    let result = part2(input);
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

fn part2(input: &str) -> usize {
    let grid = parse(input);
    let tilted = do_cycle(&grid, 1_000_000_000);
    get_load(tilted)
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn tilt_north(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = grid.len();
    let cols = grid.len();

    let mut tilted: Vec<Vec<char>> = vec![vec!['.'; cols]; rows];
    let mut boulders: Vec<Vec<usize>> = vec![vec![]; cols];

    for (row, line) in grid.into_iter().enumerate() {
        for (col, c) in line.into_iter().enumerate() {
            match c {
                'O' => boulders[col].push(row),
                '#' => tilted[row][col] = '#',
                '.' => {}
                _ => panic!("unexpected char: {}", c),
            }
        }
    }

    for (col, boulders_in_col) in boulders.into_iter().enumerate() {
        for row in boulders_in_col.into_iter() {
            // start from the position of the boulder
            let mut end_row = row;

            // go left until we hit a wall or another boulder
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

fn cycle(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut grid = grid.clone();

    for _ in 0..4 {
        grid = rotate_grid(&tilt_north(&grid))
    }
    grid
}

fn do_cycle(grid: &Vec<Vec<char>>, times: usize) -> Vec<Vec<char>> {
    let mut grid = grid.clone();

    let mut grid_to_cycle_map: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    grid_to_cycle_map.insert(grid.clone(), 0);

    let mut cycle_start: usize = 0;
    let mut cycle_end: usize = 0;

    for i in 1..=times {
        grid = cycle(&grid);

        if grid_to_cycle_map.contains_key(&grid) {
            cycle_start = *grid_to_cycle_map.get(&grid).unwrap();
            cycle_end = i;
            break;
        } else {
            grid_to_cycle_map.insert(grid.clone(), i);
        }
    }

    dbg!(cycle_start);
    dbg!(cycle_end);

    let cycle_length = cycle_end - cycle_start;
    let remaining_cycles = (times - cycle_end) % cycle_length;

    dbg!(remaining_cycles);

    for _ in 0..remaining_cycles {
        grid = cycle(&grid);
    }

    grid
}

fn rotate_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = vec![vec!['.'; grid.len()]; grid[0].len()];

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            result[col][grid[0].len() - 1 - row] = grid[row][col];
        }
    }
    result
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

        assert_eq!(part2(input), 64);
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

        let grid = parse(input);
        let result = tilt_north(&grid)
            .into_iter()
            .map(|row| row.into_iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        println!("RESULT:\n{}", result);
        println!("EXPECTED:\n{}", expected);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tilt_cycle() {
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

        let expected = [
            r#".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."#,
            r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"#,
            r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"#,
        ];

        let mut grid = parse(input);
        for (i, expected) in expected.into_iter().enumerate() {
            grid = cycle(&grid);

            let result_str = grid
                .iter()
                .map(|row| row.into_iter().collect::<String>())
                .collect::<Vec<String>>()
                .join("\n");

            println!("RESULT after cycle {}:\n{}", i + 1, result_str);
            println!("EXPECTED after cycle {}:\n{}", i + 1, expected);
            assert_eq!(result_str, *expected);
        }
    }
}
