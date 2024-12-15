use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);
}

fn part2(input: &str) -> usize {
    let (antennas, height, width) = parse_input(input);

    let mut count = 0;
    let mut freq_antinodes = HashSet::<(usize, usize)>::new();

    for frequency in antennas.keys() {
        let freq_antennas = antennas.get(frequency).unwrap();
        for i in 0..freq_antennas.len() - 1 {
            for j in i + 1..freq_antennas.len() {
                let (y1, x1) = freq_antennas[i];
                let (y2, x2) = freq_antennas[j];
                let dy = y1 as isize - y2 as isize;
                let dx = x1 as isize - x2 as isize;

                let mut antinode_1 = (y1, x1);
                while is_in_map(antinode_1, height, width) {
                    freq_antinodes.insert(antinode_1);
                    antinode_1 = (
                        antinode_1.0.wrapping_add_signed(dy),
                        antinode_1.1.wrapping_add_signed(dx),
                    );
                }

                let mut antinode_2 = (y2, x2);
                while is_in_map(antinode_2, height, width) {
                    freq_antinodes.insert(antinode_2);
                    antinode_2 = (
                        antinode_2.0.wrapping_add_signed(-dy),
                        antinode_2.1.wrapping_add_signed(-dx),
                    );
                }
            }
        }
    }
    freq_antinodes.iter().for_each(|antinode| {
        println!("antinode at: y{}, x{}", antinode.0, antinode.1);
        count += 1;
    });

    // // print map and antinodes
    // let char_map = input.lines().collect::<Vec<&str>>();
    // for y in 0..height {
    //     for x in 0..width {
    //         let c = char_map[y].chars().nth(x).unwrap();
    //         if c.is_alphanumeric() {
    //             print!("{} ", c);
    //         } else if freq_antinodes.contains(&(y, x)) {
    //             print!("# ");
    //         } else {
    //             print!(". ");
    //         }
    //     }
    //     println!();
    // }

    count
}

fn is_in_map(point: (usize, usize), height: usize, width: usize) -> bool {
    point.0 < height && point.1 < width
}

fn parse_input(input: &str) -> (HashMap<char, Vec<(usize, usize)>>, usize, usize) {
    let mut antennas = HashMap::new();
    let lines = input.lines().collect::<Vec<&str>>();

    for (y, line) in lines.iter().enumerate() {
        for (x, frequency) in line.chars().enumerate() {
            if frequency.is_ascii_alphanumeric() {
                let entry = antennas.entry(frequency).or_insert(Vec::new());
                entry.push((y, x));
                println!("{} at: {}, {}", frequency, y, x);
            }
        }
    }
    let height = lines.len();
    let width = lines[0].len();
    (antennas, height, width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example_1() {
        let input: &str = r#"..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
.......#.."#;
        assert_eq!(part2(input), 5);
    }

    #[test]
    fn it_follows_example_2() {
        let input: &str = r#"..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......#...
..........
.......#.."#;
        assert_eq!(part2(input), 8);
    }

    #[test]
    fn it_follows_example_4() {
        let input: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        assert_eq!(part2(input), 34);
    }

    #[test]
    fn it_follows_example_5() {
        let input: &str = r#"T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
.........."#;
        assert_eq!(part2(input), 9);
    }
}
