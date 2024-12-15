fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| (c as u8) - '0' as u8).collect())
        .collect();

    let map_heigth = map.len();
    let map_width = map[0].len();

    let mut summits = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 0 {
                // Found the starting point
                let mut current = (x, y);
                let mut visited = vec![vec![false; map_width]; map_heigth];
                visited[y][x] = true;

                let mut queue = vec![current];
                while !queue.is_empty() {
                    current = queue.pop().unwrap();
                    let (x, y) = current;

                    let height = map[y][x];
                    if height == 9 {
                        summits += 1;
                        continue;
                    }

                    let neighbors = vec![
                        (x.wrapping_sub(1), y),
                        (x.wrapping_add(1), y),
                        (x, y.wrapping_sub(1)),
                        (x, y.wrapping_add(1)),
                    ];

                    for (nx, ny) in neighbors {
                        if nx < map_width
                            && ny < map_heigth
                            && !visited[ny][nx]
                            && map[ny][nx].wrapping_sub(height) == 1
                        {
                            visited[ny][nx] = true;
                            queue.push((nx, ny));
                        }
                    }
                }
            }
        }
    }

    summits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example_1() {
        let input: &str = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;
        assert_eq!(part1(input), 36);
    }

    #[test]
    fn it_follows_example_2() {
        let input: &str = r#"0123
1234
8765
9876"#;
        assert_eq!(part1(input), 1);
    }
}
