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
                let (area, sides) = explore_area((x, y), &map, &mut visited);
                println!(
                    "region of {} [{:2},{:2}] has area {:3}, and sides {:2}, total: {}",
                    map[y][x],
                    x,
                    y,
                    area,
                    sides,
                    area * sides
                );
                result += area * sides;
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

    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut min_y = usize::MAX;
    let mut max_y = 0;

    let mut region_map = vec![vec![false; map[0].len()]; map.len()];
    visited[pos.1][pos.0] = true;

    while let Some(pos) = stack.pop() {
        area += 1;
        region_map[pos.1][pos.0] = true;

        min_x = min_x.min(pos.0);
        max_x = max_x.max(pos.0);
        min_y = min_y.min(pos.1);
        max_y = max_y.max(pos.1);

        let neighbours = vec![
            (pos.0, pos.1.wrapping_sub(1)),
            (pos.0.wrapping_add(1), pos.1),
            (pos.0, pos.1.wrapping_add(1)),
            (pos.0.wrapping_sub(1), pos.1),
        ];

        for neighbour in neighbours {
            if is_inside(neighbour, map)
                && is_same_region(pos, neighbour, map)
                && !visited[neighbour.1][neighbour.0]
            {
                visited[neighbour.1][neighbour.0] = true;
                stack.push(neighbour);
            }
        }
    }

    let mut sides = 0;

    // we want to do horizontal and vertical passes to count the sides
    for y in min_y..=max_y {
        // horizontal upper edge pass
        let mut is_side = false;
        for x in min_x..=max_x {
            let curr_pos_up = (x, y.wrapping_sub(1));
            if region_map[y][x]
                && (!is_inside(curr_pos_up, map) || !region_map[curr_pos_up.1][curr_pos_up.0])
            {
                if !is_side {
                    sides += 1;
                }
                is_side = true;
            } else {
                is_side = false;
            }
        }

        // horizontal lower edge pass
        let mut is_side = false;
        for x in min_x..=max_x {
            let curr_pos_down = (x, y.wrapping_add(1));
            if region_map[y][x]
                && (!is_inside(curr_pos_down, map) || !region_map[curr_pos_down.1][curr_pos_down.0])
            {
                if !is_side {
                    sides += 1;
                }
                is_side = true;
            } else {
                is_side = false;
            }
        }
    }

    for x in min_x..=max_x {
        // horizontal upper edge pass
        let mut is_side = false;
        for y in min_y..=max_y {
            let curr_pos_right = (x.wrapping_add(1), y);
            if region_map[y][x]
                && (!is_inside(curr_pos_right, map)
                    || !region_map[curr_pos_right.1][curr_pos_right.0])
            {
                if !is_side {
                    sides += 1;
                }
                is_side = true;
            } else {
                is_side = false;
            }
        }

        // horizontal lower edge pass
        let mut is_side = false;
        for y in min_y..=max_y {
            let curr_pos_left = (x.wrapping_sub(1), y);
            if region_map[y][x]
                && (!is_inside(curr_pos_left, map) || !region_map[curr_pos_left.1][curr_pos_left.0])
            {
                if !is_side {
                    sides += 1;
                }
                is_side = true;
            } else {
                is_side = false;
            }
        }
    }

    (area, sides)
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
        assert_eq!(part1(input), 80);
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
        assert_eq!(part1(input), 1206);
    }

    #[test]
    fn it_follows_example_3() {
        let input: &str = r#"EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"#;
        assert_eq!(part1(input), 236);
    }

    #[test]
    fn it_follows_example_4() {
        let input: &str = r#"AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"#;

        assert_eq!(part1(input), 368);
    }
}
