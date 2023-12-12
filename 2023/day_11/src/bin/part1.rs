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

struct Observation {
    galaxies: Vec<(usize, usize)>,
    rows_empty: Vec<bool>,
    cols_empty: Vec<bool>,

    rows_empty_count: Vec<usize>,
    cols_empty_count: Vec<usize>,
}
impl Observation {
    fn new(input: &str) -> Self {
        let mut galaxies = Vec::new();
        let lines = input.lines().collect::<Vec<_>>();

        let n_rows = lines.len();
        let n_cols = lines[0].len();
        let mut rows_empty = vec![true; n_rows];
        let mut cols_empty = vec![true; n_cols];

        for (row, line) in lines.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push((row, col));
                    rows_empty[row] = false;
                    cols_empty[col] = false;
                }
            }
        }

        let mut rows_empty_count = Vec::with_capacity(n_rows);
        let mut cols_empty_count = Vec::with_capacity(n_cols);
        let mut empty_rows_count = 0;
        for i in 0..n_rows {
            if rows_empty[i] {
                empty_rows_count += 1;
            }
            rows_empty_count.push(empty_rows_count);
        }
        let mut empty_cols_count = 0;
        for i in 0..n_cols {
            if cols_empty[i] {
                empty_cols_count += 1;
            }
            cols_empty_count.push(empty_cols_count);
        }

        Observation {
            galaxies,
            rows_empty,
            cols_empty,
            rows_empty_count,
            cols_empty_count,
        }
    }

    fn distance_between(&self, galaxy1: (usize, usize), galaxy2: (usize, usize)) -> usize {
        let row_min = galaxy1.0.min(galaxy2.0);
        let row_max = galaxy1.0.max(galaxy2.0);
        let col_min = galaxy1.1.min(galaxy2.1);
        let col_max = galaxy1.1.max(galaxy2.1);

        let row_distance =
            row_max - row_min + self.rows_empty_count[row_max] - self.rows_empty_count[row_min];
        let col_distance =
            col_max - col_min + self.cols_empty_count[col_max] - self.cols_empty_count[col_min];
        row_distance + col_distance
    }

    fn sum_distance_between_all_galaxies(&self) -> usize {
        let mut sum: usize = 0;
        for i in 0..self.galaxies.len() - 1 {
            for j in i + 1..self.galaxies.len() {
                sum += self.distance_between(self.galaxies[i], self.galaxies[j]);
            }
        }
        sum
    }
}

fn part1(_input: &str) -> usize {
    let observation = Observation::new(_input);
    observation.sum_distance_between_all_galaxies()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

    #[test]
    fn it_follows_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 374);
    }

    #[test]
    fn observation_new() {
        let galaxies = vec![
            (0, 3),
            (1, 7),
            (2, 0),
            (4, 6),
            (5, 1),
            (6, 9),
            (8, 7),
            (9, 0),
            (9, 4),
        ];

        let mut rows_empty = vec![false; 10];
        rows_empty[3] = true;
        rows_empty[7] = true;

        let mut cols_empty = vec![false; 10];
        cols_empty[2] = true;
        cols_empty[5] = true;
        cols_empty[8] = true;

        let rows_empty_count = vec![0, 0, 0, 1, 1, 1, 1, 2, 2, 2];
        let cols_empty_count = vec![0, 0, 1, 1, 1, 2, 2, 2, 3, 3];

        let obs = Observation {
            galaxies,
            rows_empty,
            cols_empty,
            rows_empty_count,
            cols_empty_count,
        };

        let created = Observation::new(EXAMPLE_INPUT);
        assert_eq!(created.galaxies, obs.galaxies);
        assert_eq!(created.rows_empty, obs.rows_empty);
        assert_eq!(created.cols_empty, obs.cols_empty);
        assert_eq!(created.rows_empty_count, obs.rows_empty_count);
        assert_eq!(created.cols_empty_count, obs.cols_empty_count);
    }

    #[test]
    fn test_distance() {}
}
