use roots::find_roots_quadratic;

fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);
}

fn part2(input: &str) -> usize {
    let (time, distance) = parse(input);
    let result = ways_to_win_race(time, distance);
    result
}

fn parse(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let races_time: Vec<&str> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect();
    let races_distance: Vec<&str> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect();
    let race_time: usize = races_time.join("").parse().unwrap();
    let race_distance: usize = races_distance.join("").parse().unwrap();

    (race_time, race_distance)
}

fn ints_in_range(min: f64, max: f64) -> usize {
    let umin = min.ceil() as usize;
    let umax = max.floor() as usize;

    let mut res = umax - umin + 1;
    if max == max.floor() {
        res -= 1;
    }
    if min == min.ceil() {
        res -= 1;
    }

    res
}

fn ways_to_win_race(time: usize, distance: usize) -> usize {
    let a2 = -1.0;
    let a1 = time as f64;
    let a0 = -(distance as f64);

    match find_roots_quadratic(a2, a1, a0) {
        roots::Roots::Two([r1, r2]) => ints_in_range(r1, r2),
        x => panic!("Unexpected number of roots: {:?}", x),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200
"#;

        let result = super::part2(input);
        assert_eq!(result, 71503);
    }

    #[test]
    fn test_parse() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200
"#;

        let result = super::parse(input);
        assert_eq!(result, (71530, 940200));
    }

    #[test]
    fn it_works2() {
        let inputs = vec![(71530, 940200, 71503)];

        for (time, distance, result) in inputs {
            assert_eq!(super::ways_to_win_race(time, distance), result);
        }
    }
}
