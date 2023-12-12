use roots::find_roots_quadratic;

fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|(time, distance)| ways_to_win_race(time, distance))
        .product()
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut lines = input.lines();
    let races_time: Vec<usize> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let races_distance: Vec<usize> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let races = races_time
        .into_iter()
        .zip(races_distance.into_iter())
        .collect();
    races
}

fn ints_in_range(min: f32, max: f32) -> usize {
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
    let a1 = time as f32;
    let a0 = -(distance as f32);

    match find_roots_quadratic(a2, a1, a0) {
        roots::Roots::Two([r1, r2]) => ints_in_range(r1, r2),
        x => panic!("Unexpected number of roots: {:?}", x),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200
"#;

        let result = super::part1(input);
        assert_eq!(result, 288);
    }

    #[test]
    fn test_parse() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200
"#;

        let result = super::parse(input);
        assert_eq!(result, vec![(7, 9), (15, 40), (30, 200)]);
    }

    #[test]
    fn it_works2() {
        let inputs = vec![(7, 9, 4), (15, 40, 8), (30, 200, 9)];

        for (time, distance, result) in inputs {
            assert_eq!(super::ways_to_win_race(time, distance), result);
        }
    }
}
