use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
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

                let antinode_1 = (y1.wrapping_add_signed(dy), x1.wrapping_add_signed(dx));
                let antinode_2 = (y2.wrapping_add_signed(-dy), x2.wrapping_add_signed(-dx));

                if is_in_map(antinode_1, height, width) {
                    freq_antinodes.insert(antinode_1);
                }
                if is_in_map(antinode_2, height, width) {
                    freq_antinodes.insert(antinode_2);
                }
            }
        }
    }
    freq_antinodes.iter().for_each(|antinode| {
        println!("antinode at: y{}, x{}", antinode.0, antinode.1);
        count += 1;
    });

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
.........."#;
        assert_eq!(part1(input), 2);
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
.........."#;
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn it_follows_example_3() {
        let input: &str = r#"..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......A...
..........
.........."#;
        assert_eq!(part1(input), 4);
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
        assert_eq!(part1(input), 14);
    }
}
