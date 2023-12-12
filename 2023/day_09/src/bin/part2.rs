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

fn part1(input: &str) -> isize {
    parse_input(input)
        .into_iter()
        .map(|readings| extrapolate_next(readings))
        .sum()
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Vec<isize> {
    line.split_ascii_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

fn all_zero(readings: &[isize]) -> bool {
    readings.iter().all(|&reading| reading == 0)
}

fn extrapolate_next(readings: Vec<isize>) -> isize {
    let (diff_vec, iterations) = build_diff_vec(readings);

    let diff_vec = &diff_vec[..iterations];
    dbg!(diff_vec);
    let mut running_estimate = 0;
    for i in (0..iterations).rev() {
        dbg!(running_estimate, diff_vec[i]);
        running_estimate = diff_vec[i] - running_estimate;
    }
    dbg!(running_estimate);
    running_estimate
}

fn build_diff_vec(mut readings: Vec<isize>) -> (Vec<isize>, usize) {
    let len = readings.len();
    let mut iteration = 1;
    loop {
        for i in (iteration..len).rev() {
            readings[i] = readings[i] - readings[i - 1];
        }

        if all_zero(&readings[iteration..]) {
            break;
        }

        iteration += 1;
    }
    return (readings, iteration);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const EXAMPLE_INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn it_follows_example() {
        assert_eq!(part1(EXAMPLE_INPUT), 2);
    }

    #[test]
    fn it_parses_input() {
        let test_cases = vec![(
            EXAMPLE_INPUT,
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45],
            ],
        )];

        for (input, expected) in test_cases {
            assert_eq!(parse_input(input), expected);
        }
    }

    #[test]
    fn build_diff_vec_correctly() {
        let test_cases = vec![
            (vec![0, 3, 6, 9, 12, 15], (vec![0, 3, 0, 0, 0, 0], 2)),
            (vec![1, 3, 6, 10, 15, 21], (vec![1, 2, 1, 0, 0, 0], 3)),
            (vec![10, 13, 16, 21, 30, 45], (vec![10, 3, 0, 2, 0, 0], 4)),
        ];

        for (input, expected) in test_cases {
            assert_eq!(build_diff_vec(input), expected);
        }
    }

    #[test]
    fn estimates_correctly() {
        let test_cases = vec![
            (vec![0, 3, 6, 9, 12, 15], -3),
            (vec![1, 3, 6, 10, 15, 21], 0),
            (vec![10, 13, 16, 21, 30, 45], 5),
        ];

        for (input, expected) in test_cases {
            assert_eq!(extrapolate_next(input), expected);
        }
    }

    #[test]
    fn into_iter_on_mut_vec_slice() {
        let mut vec = vec![1, 2, 3, 4, 5];
        let slice = &mut vec[..3];

        for (i, n) in slice.into_iter().enumerate() {
            *n += i as isize;
        }

        assert_eq!(vec, vec![1, 3, 5, 4, 5]);
    }
}
