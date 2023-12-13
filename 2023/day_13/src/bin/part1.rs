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
    let problems = input.split("\n\n").collect::<Vec<_>>();
    let mut score = 0;

    for problem in problems {
        let lines = problem.lines().collect::<Vec<_>>();
        let rows = lines.len();
        let cols = lines[0].len();
        let grid = lines
            .iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut found = 0;

        for col in 1..cols {
            if is_mirrored_on_col(&grid, col) {
                // println!("found mirrored on col {} for problem: \n{}", col, problem);
                score += col;
                found += 1;
            }
        }

        for row in 1..rows {
            if is_mirrored_on_row(&grid, row) {
                // println!("found mirrored on row {} for problem: \n{}", row, problem);
                score += row * 100;
                found += 1;
            }
        }

        if found == 0 {
            println!("found nothing for problem: \n{}", problem);
        } else if found > 1 {
            println!("found {} reflections for problem: \n{}", found, problem);
        }
    }

    score
}
fn is_mirrored_on_col(grid: &Vec<Vec<char>>, col: usize) -> bool {
    let grid_cols = grid[0].len();
    let cols = col.min(grid_cols - col);

    for row in grid {
        for i in 0..cols {
            if row[col - i - 1] != row[col + i] {
                return false;
            }
        }
    }
    cols > 0
}

fn is_mirrored_on_row(grid: &Vec<Vec<char>>, row: usize) -> bool {
    let grid_rows = grid.len();
    let rows = row.min(grid_rows - row);

    for i in 0..rows {
        if grid[row - i - 1] != grid[row + i] {
            return false;
        }
    }
    rows > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

    #[test]
    fn it_follows_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 405);
    }

    #[test]
    fn another_example() {
        assert_eq!(
            part1(
                r#"#.##.#....#.##.##
##..##....##..###
..##........##..#
.####......####..
.#..#.####.#..#.#
.##.#.####.#.##..
################.
......####.......
#....######....##
#######..#######.
.......##.......#
..##...##...##...
......####.......
#....#.##.#....#.
.####..##..####.#"#
            ),
            8
        );
    }
}
