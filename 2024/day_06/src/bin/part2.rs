use std::{collections::HashSet, usize, vec};

fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);
}

fn part2(input: &str) -> usize {
    let mut obstacle_map: Vec<Vec<bool>> = Vec::new();

    let mut starting_pos = (usize::MAX, usize::MAX);
    let starting_dir = (-1, 0); // Up
    for (y, line) in input.lines().enumerate() {
        obstacle_map.push(vec![false; line.len()]);
        for (x, c) in line.chars().enumerate() {
            let is_obstacle = match c {
                '#' => true,
                '.' => false,
                '^' => {
                    starting_pos = (y, x);
                    false
                }
                _ => panic!("Unexpected character: {}", c),
            };
            obstacle_map[y][x] = is_obstacle;
        }
    }

    let mut visited: HashSet<(usize, usize)> =
        get_visited(&obstacle_map, starting_pos, starting_dir);
    visited.remove(&starting_pos);

    let mut count = 0;

    // we put a obstacle in each visited cell and check if it causes a cycle
    // all the other cells would have no impact on the cycle generation
    for (y, x) in visited.into_iter() {
        obstacle_map[y][x] = true;
        if causes_cycle(&obstacle_map, starting_pos, starting_dir) {
            count += 1;
            println!("Cycle detected by adding obstacle at ({}, {})", y, x);
        } else {
            println!("No cycle detected by adding obstacle at ({}, {})", y, x);
        }
        obstacle_map[y][x] = false;
    }

    count
}

fn is_out_of_bounds(pos: (usize, usize), map: &Vec<Vec<bool>>) -> bool {
    pos.0 >= map.len() || pos.1 >= map[0].len()
}

fn get_visited(
    map: &Vec<Vec<bool>>,
    starting_pos: (usize, usize),
    starting_dir: (isize, isize),
) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut player_pos = starting_pos;
    let mut player_dir = starting_dir;
    visited.insert(player_pos);

    loop {
        let next_pos = (
            player_pos.0.wrapping_add_signed(player_dir.0),
            player_pos.1.wrapping_add_signed(player_dir.1),
        );
        if is_out_of_bounds(next_pos, &map) {
            break;
        }

        if map[next_pos.0][next_pos.1] {
            // turn right
            player_dir = match player_dir {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => panic!("Unexpected direction: {:?}", player_dir),
            };
            continue;
        }

        player_pos = next_pos;

        if !visited.contains(&player_pos) {
            visited.insert(player_pos);
        }
    }

    visited
}
fn causes_cycle(
    map: &Vec<Vec<bool>>,
    starting_pos: (usize, usize),
    starting_dir: (isize, isize),
) -> bool {
    let mut visited: Vec<Vec<u8>> = vec![vec![0; map[0].len()]; map.len()];
    let mut player_pos = starting_pos;
    let mut player_dir = starting_dir;

    loop {
        let next_pos = (
            player_pos.0.wrapping_add_signed(player_dir.0),
            player_pos.1.wrapping_add_signed(player_dir.1),
        );
        if is_out_of_bounds(next_pos, &map) {
            return false;
        }

        if map[next_pos.0][next_pos.1] {
            // turn right
            visited[player_pos.0][player_pos.1] |= dir_to_flag(player_dir);
            player_dir = match player_dir {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => panic!("Unexpected direction: {:?}", player_dir),
            };
            continue;
        }

        let flag = dir_to_flag(player_dir);
        if was_visited(visited[player_pos.0][player_pos.1], player_dir) {
            print_map(map, visited, starting_pos, player_pos);

            return true;
        }

        visited[player_pos.0][player_pos.1] |= flag;
        player_pos = next_pos;
    }
}

fn dir_to_flag(dir: (isize, isize)) -> u8 {
    match dir {
        (-1, 0) => 1,
        (0, 1) => 2,
        (1, 0) => 4,
        (0, -1) => 8,
        _ => panic!("Unexpected direction: {:?}", dir),
    }
}

fn was_visited(flag: u8, dir: (isize, isize)) -> bool {
    flag & dir_to_flag(dir) != 0
}

fn print_map(
    map: &Vec<Vec<bool>>,
    visited: Vec<Vec<u8>>,
    player_start: (usize, usize),
    player_pos: (usize, usize),
) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if player_pos == (y, x) {
                print!("E ");
            } else if player_start == (y, x) {
                print!("S ");
            } else if *cell {
                print!("# ");
            } else if visited[y][x] == 0 {
                print!(". ");
            } else {
                let horizontal = visited[y][x] & 10 != 0;
                let vertical = visited[y][x] & 5 != 0;
                if horizontal {
                    print!("{}", if vertical { "╋" } else { "━" });

                    if x + 1 < map[y].len() && visited[y][x + 1] & 10 != 0 {
                        print!("━");
                    } else {
                        print!(".");
                    }
                } else if vertical {
                    print!("┃ ");
                } else {
                    print!("? ");
                }
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(part2(input), 6);
    }
}
