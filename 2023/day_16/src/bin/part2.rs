fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);
}

fn part2(input: &str) -> usize {
    let map = parse_map(input);
    let mut max_energized = 0;

    let rows = map.len();
    let cols = map[0].len();

    for i in 0..rows {
        let start_left = (i, 0);
        let energized = energize(&map, start_left, RIGHT);
        max_energized = max_energized.max(count_energized(&energized));

        let start_right = (i, cols - 1);
        let energized = energize(&map, start_right, LEFT);
        max_energized = max_energized.max(count_energized(&energized));
    }

    for j in 0..cols {
        let start_up = (0, j);
        let energized = energize(&map, start_up, DOWN);
        max_energized = max_energized.max(count_energized(&energized));

        let start_down = (rows - 1, j);
        let energized = energize(&map, start_down, UP);
        max_energized = max_energized.max(count_energized(&energized));
    }

    max_energized
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

const RIGHT: (isize, isize) = (0, 1);
const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);
const UP: (isize, isize) = (-1, 0);

const _DIRECTIONS: [(isize, isize); 4] = [RIGHT, DOWN, LEFT, UP];

fn dir_to_index(dir: (isize, isize)) -> usize {
    match dir {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => panic!("Invalid direction"),
    }
}

fn energize(map: &[Vec<char>], start: (usize, usize), dir: (isize, isize)) -> Vec<Vec<bool>> {
    let mut energized_in_direction: Vec<Vec<Vec<bool>>> =
        vec![vec![vec![false; 4]; map[0].len()]; map.len()];

    energized_in_direction[start.0][start.1][dir_to_index(dir)] = true;

    energize_rec(map, start, dir, &mut energized_in_direction);

    let energized: Vec<Vec<bool>> = energized_in_direction
        .iter()
        .map(|row| row.iter().map(|col| col.iter().any(|&cell| cell)).collect())
        .collect();
    energized
}

fn count_energized(energized: &Vec<Vec<bool>>) -> usize {
    energized
        .iter()
        .map(|row| row.iter().filter(|&&cell| cell).count())
        .sum()
}

fn in_map_bounds(map: &[Vec<char>], pos: (usize, usize)) -> bool {
    pos.0 < map.len() && pos.1 < map[0].len()
}

fn pos_after_move(
    map: &[Vec<char>],
    pos: (usize, usize),
    dir: (isize, isize),
) -> Option<(usize, usize)> {
    let new_ipos = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
    if new_ipos.0 >= 0 && new_ipos.1 >= 0 {
        let new_pos = (new_ipos.0 as usize, new_ipos.1 as usize);
        if in_map_bounds(map, new_pos) {
            return Some(new_pos);
        }
    }
    None
}

fn energize_rec(
    map: &[Vec<char>],
    pos: (usize, usize),
    dir: (isize, isize),
    mut energized_in_direction: &mut Vec<Vec<Vec<bool>>>,
) -> () {
    let mirror = map[pos.0][pos.1];
    match mirror {
        '.' => {
            let next_pos = pos_after_move(map, pos, dir);
            if let Some(next_pos) = next_pos {
                let explored_dir =
                    &mut energized_in_direction[next_pos.0][next_pos.1][dir_to_index(dir)];

                if !(*explored_dir) {
                    *explored_dir = true;
                    energize_rec(map, next_pos, dir, &mut energized_in_direction);
                }
            }
        }
        '/' | '\\' => {
            let mut next_dir = match dir {
                RIGHT => UP,
                DOWN => LEFT,
                LEFT => DOWN,
                UP => RIGHT,
                _ => panic!("Invalid direction"),
            };

            if mirror == '\\' {
                next_dir = (-next_dir.0, -next_dir.1);
            }

            let next_pos = pos_after_move(map, pos, next_dir);
            if let Some(next_pos) = next_pos {
                let explored_dir =
                    &mut energized_in_direction[next_pos.0][next_pos.1][dir_to_index(next_dir)];

                if !(*explored_dir) {
                    *explored_dir = true;
                    energize_rec(map, next_pos, next_dir, &mut energized_in_direction);
                }
            }
        }
        '|' | '-' => {
            let horizontal = mirror == '-';
            let is_mirror_ignored = match dir {
                RIGHT => horizontal,
                DOWN => !horizontal,
                LEFT => horizontal,
                UP => !horizontal,
                _ => panic!("Invalid direction"),
            };
            if is_mirror_ignored {
                let next_pos = pos_after_move(map, pos, dir);
                if let Some(next_pos) = next_pos {
                    let explored_dir =
                        &mut energized_in_direction[next_pos.0][next_pos.1][dir_to_index(dir)];

                    if !(*explored_dir) {
                        *explored_dir = true;
                        energize_rec(map, next_pos, dir, &mut energized_in_direction);
                    }
                }
            } else {
                let next_dirs = match horizontal {
                    true => [LEFT, RIGHT],
                    false => [UP, DOWN],
                };
                let mut to_explore = [false, false];

                for (i, next_dir) in next_dirs.into_iter().enumerate() {
                    let next_pos = pos_after_move(map, pos, next_dir);
                    if let Some(next_pos) = next_pos {
                        let explored_dir = &mut energized_in_direction[next_pos.0][next_pos.1]
                            [dir_to_index(next_dir)];

                        // mark directions as explored
                        if !(*explored_dir) {
                            *explored_dir = true;
                            to_explore[i] = true;
                        }
                    }
                }

                // actually explore
                for (i, next_dir) in next_dirs.into_iter().enumerate() {
                    if to_explore[i] {
                        let next_pos = pos_after_move(map, pos, next_dir).unwrap();
                        energize_rec(map, next_pos, next_dir, &mut energized_in_direction);
                    }
                }
            }
        }
        _ => panic!("Invalid mirror"),
    }
}

fn energized_to_str(energized: &Vec<Vec<bool>>) -> String {
    let energized_str = energized
        .iter()
        .map(|row| {
            row.iter()
                .map(|&cell| if cell { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n");
    energized_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let map: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

        assert_eq!(part2(map), 51);
    }

    #[test]
    fn it_energizes_correctly() {
        let input_map: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

        let expected_energized = r#"######....
.#...#....
.#...#####
.#...##...
.#...##...
.#...##...
.#..####..
########..
.#######..
.#...#.#.."#;

        let map = parse_map(input_map);
        let energized: Vec<Vec<bool>> = energize(&map, (0, 0), RIGHT);

        assert_eq!(energized_to_str(&energized), expected_energized);
    }

    #[test]
    fn it_handles_changing_direction_immediately() {
        let input_map: &str = r#"\......
..././.
\..|...
......."#;

        let expected_energized = r#"#....#.
#..###.
####...
...#..."#;

        let map = parse_map(input_map);
        let energized: Vec<Vec<bool>> = energize(&map, (0, 0), RIGHT);
        let energized_str = energized_to_str(&energized);

        println!("energized:\n{}", energized_str);
        println!("expected:\n{}", expected_energized);

        assert_eq!(energized_str, expected_energized);
    }
}
