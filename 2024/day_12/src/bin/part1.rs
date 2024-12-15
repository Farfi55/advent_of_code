use std::usize;

fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut result = 0;

    let height = map.len();
    let width = map[0].len();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if !visited[y][x] {
                let (area, perimeter) = explore_area((x, y), &map, &mut visited);
                println!(
                    "region of {} [{},{}] has area {}, and perimeter {}",
                    map[y][x], x, y, area, perimeter
                );
                result += area * perimeter;
            }
        }
    }
    result
}

fn is_inside(pos: (usize, usize), map: &Vec<Vec<char>>) -> bool {
    pos.1 < map.len() && pos.0 < map[0].len()
}

fn is_same_region(pos1: (usize, usize), pos2: (usize, usize), map: &Vec<Vec<char>>) -> bool {
    map[pos1.1][pos1.0] == map[pos2.1][pos2.0]
}

fn explore_area(
    pos: (usize, usize),
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> (usize, usize) {
    let mut stack = vec![pos];
    let mut area = 0;
    let mut perimeter = 0;
    visited[pos.1][pos.0] = true;

    while let Some(pos) = stack.pop() {
        area += 1;

        let neighbours = vec![
            (pos.0, pos.1.wrapping_sub(1)),
            (pos.0.wrapping_add(1), pos.1),
            (pos.0, pos.1.wrapping_add(1)),
            (pos.0.wrapping_sub(1), pos.1),
        ];

        for neighbour in neighbours {
            if !is_inside(neighbour, map) || !is_same_region(pos, neighbour, map) {
                perimeter += 1
            } else if !visited[neighbour.1][neighbour.0] {
                visited[neighbour.1][neighbour.0] = true;
                stack.push(neighbour);
            }
        }
    }

    (area, perimeter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example_1() {
        let input: &str = r#"AAAA
BBCD
BBCC
EEEC"#;
        assert_eq!(part1(input), 140);
    }

    #[test]
    fn it_follows_example_2() {
        let input: &str = r#"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"#;
        assert_eq!(part1(input), 1930);
    }
}
