use std::{usize, vec};

fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    let mut obstacle_map: Vec<Vec<bool>> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();
    let mut visited_count = 0;

    let mut player_pos = (usize::MAX, usize::MAX);
    let mut player_dir = (-1, 0); // Up
    for (y, line) in input.lines().enumerate() {
        obstacle_map.push(vec![false; line.len()]);
        visited.push(vec![false; line.len()]);
        for (x, c) in line.chars().enumerate() {
            let is_obstacle = match c {
                '#' => true,
                '.' => false,
                '^' => {
                    player_pos = (y, x);
                    visited[y][x] = true;
                    visited_count += 1;
                    false
                }
                _ => panic!("Unexpected character: {}", c),
            };
            obstacle_map[y][x] = is_obstacle;
        }
    }

    loop {
        let next_pos = (
            player_pos.0.wrapping_add_signed(player_dir.0),
            player_pos.1.wrapping_add_signed(player_dir.1),
        );
        if is_out_of_bounds(next_pos, &obstacle_map) {
            break;
        }

        if obstacle_map[next_pos.0][next_pos.1] {
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

        if !visited[player_pos.0][player_pos.1] {
            visited[player_pos.0][player_pos.1] = true;
            visited_count += 1;
        }
    }

    visited_count
}

fn is_out_of_bounds(pos: (usize, usize), map: &Vec<Vec<bool>>) -> bool {
    pos.0 >= map.len() || pos.1 >= map[0].len()
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
        assert_eq!(part1(input), 41);
    }
}
