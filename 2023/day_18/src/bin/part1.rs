fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    let operations = parse(input);
    let (start, trench) = trench(&operations);
    let digged = digged(start, &trench);
    count_digged(&digged)
}

#[derive(Debug, PartialEq)]
struct Operation {
    direction: (isize, isize),
    distance: usize,
    color: (u8, u8, u8),
}

fn parse(input: &str) -> Vec<Operation> {
    input.lines().into_iter().map(parse_line).collect()
}

const RIGHT: (isize, isize) = (0, 1);
const LEFT: (isize, isize) = (0, -1);
const UP: (isize, isize) = (-1, 0);
const DOWN: (isize, isize) = (1, 0);

fn parse_line(line: &str) -> Operation {
    let mut parts = line.split_whitespace();
    let direction = parts.next().unwrap();
    let distance = parts.next().unwrap();
    let color = parts.next().unwrap();

    let direction = match direction {
        "R" => RIGHT,
        "L" => LEFT,
        "U" => UP,
        "D" => DOWN,
        _ => panic!("Unknown direction: {}", direction),
    };

    let distance = distance.parse().unwrap();

    // 0dc571
    // 0d, c5, 71
    // 13, 197, 113
    let color = color[2..color.len() - 1]
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            let chunk = std::str::from_utf8(chunk).unwrap();
            u8::from_str_radix(chunk, 16).unwrap()
        })
        .collect::<Vec<u8>>();

    Operation {
        direction,
        distance,
        color: (color[0], color[1], color[2]),
    }
}

fn get_bounds(operations: &Vec<Operation>) -> (usize, usize, usize, usize) {
    let mut right = 1;
    let mut left = 1;
    let mut up = 1;
    let mut down = 1;

    for operation in operations {
        match operation.direction {
            RIGHT => right += operation.distance,
            LEFT => left += operation.distance,
            UP => up += operation.distance,
            DOWN => down += operation.distance,
            _ => panic!("Unknown direction: {:?}", operation.direction),
        }
    }

    (right, left, up, down)
}

fn trench(operations: &Vec<Operation>) -> ((usize, usize), Vec<Vec<(u8, u8, u8)>>) {
    let (right, left, up, down) = get_bounds(operations);
    let width = right + left + 1;
    let height = up + down + 1;

    let mut trench = vec![vec![(0u8, 0u8, 0u8); width]; height];

    let start = (up, left);
    let mut y = up;
    let mut x = left;

    for operation in operations {
        let dist = operation.distance;
        let (dy, dx) = operation.direction;

        for _ in 0..dist {
            x = (x as isize + dx) as usize;
            y = (y as isize + dy) as usize;

            trench[y][x] = operation.color;
        }
    }
    (start, trench)
}

fn to_string(trench: &Vec<Vec<(u8, u8, u8)>>) -> String {
    let mut result = String::new();

    for row in trench {
        for color in row {
            if *color == (0, 0, 0) {
                result.push('.');
            } else {
                result.push('#');
            }
        }
        result.push('\n');
    }

    result
}

fn digged(start: (usize, usize), trench: &Vec<Vec<(u8, u8, u8)>>) -> Vec<Vec<(u8, u8, u8)>> {
    let possible_fills: Vec<(usize, usize)> = Vec::new();

    for dir in [RIGHT, LEFT, UP, DOWN, (1, 1), (1, -1), (-1, 1), (-1, -1)] {
        let y = (start.0 as isize + dir.0) as usize;
        let x = (start.1 as isize + dir.1) as usize;

        if trench[y][x] == (0, 0, 0) {
            let digged: Option<Vec<Vec<(u8, u8, u8)>>> = try_dig((y, x), trench);

            if let Some(digged) = digged {
                return digged;
            }
        }
    }

    panic!("No possible fill found");
}

fn try_dig(
    start: (usize, usize),
    trench: &Vec<Vec<(u8, u8, u8)>>,
) -> Option<Vec<Vec<(u8, u8, u8)>>> {
    let mut digged = trench.clone();
    let mut queue = std::collections::VecDeque::new();

    queue.push_back(start);
    digged[start.0][start.1] = (255, 255, 255);

    while let Some((y, x)) = queue.pop_front() {
        if y == 0 || x == 0 || y == trench.len() - 1 || x == trench[0].len() - 1 {
            return None;
        }

        for dir in [LEFT, RIGHT, UP, DOWN] {
            let y = (y as isize + dir.0) as usize;
            let x = (x as isize + dir.1) as usize;

            if digged[y][x] == (0, 0, 0) {
                digged[y][x] = (255, 255, 255);
                queue.push_back((y, x));
            }
        }
    }

    Some(digged)
}

fn count_digged(digged: &Vec<Vec<(u8, u8, u8)>>) -> usize {
    digged.iter().flatten().filter(|c| **c != (0, 0, 0)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

        assert_eq!(part1(input), 62);
    }

    #[test]
    fn parse_works() {
        let input: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)"#;

        let expected = vec![
            Operation {
                direction: RIGHT,
                distance: 6,
                color: (112, 199, 16),
            },
            Operation {
                direction: DOWN,
                distance: 5,
                color: (13, 197, 113),
            },
            Operation {
                direction: LEFT,
                distance: 2,
                color: (87, 19, 240),
            },
            Operation {
                direction: DOWN,
                distance: 2,
                color: (210, 192, 129),
            },
        ];

        assert_eq!(parse(input), expected);
    }

    #[test]
    fn it_digs_correctly() {
        let input: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

        let expected_trench: &str = r#"#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######"#;

        let expected_digged: &str = r#"#######
#######
#######
..#####
..#####
#######
#####..
#######
.######
.######"#;

        let operations = parse(input);
        let (start, trench) = trench(&operations);

        let trench_str = to_string(&trench);

        print!("trench:\n{}", trench_str);
        print!("expected_trench:\n{}", expected_trench);

        let digged = digged(start, &trench);
        let digged_str = to_string(&digged);
        print!("digged:\n{}", digged_str);
        print!("expected_digged:\n{}", expected_digged);
    }
}
