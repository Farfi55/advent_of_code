fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    let target: &str = "XMAS";
    let target_rev: String = target.chars().rev().collect();

    let mut count: usize = 0;
    let mut lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    println!("ORIGINAL");
    for line in &lines {
        println!("{}", line);
    }

    count += count_instances_of(&lines, &target);

    lines = rotate(&lines);

    println!("ROTATED");
    for line in &lines {
        println!("{}", line);
    }

    count += count_instances_of(&lines, &target);

    count
}

fn rotate(lines: &Vec<String>) -> Vec<String> {
    let height = lines.len();
    let width = lines[0].len();

    let mut rotated: Vec<String> = vec![String::new(); width];

    for i in 0..height {
        for j in 0..width {
            let c = lines[j].chars().nth(height - i - 1).unwrap();
            rotated[i].push(c);
        }
    }
    rotated
}

fn count_instances_of(lines: &Vec<String>, target: &str) -> usize {
    let mut count = 0;
    let target_rev: String = target.chars().rev().collect();

    for (i, line) in lines.iter().enumerate() {
        let maches = line.match_indices(target).collect::<Vec<_>>();
        count += maches.len();
        for (j, _) in maches {
            println!("found {} at ({}, {})", target, i, j);
        }

        let maches_rev = line.match_indices(&target_rev).collect::<Vec<_>>();
        count += maches_rev.len();
        for (j, _) in maches_rev {
            println!("found {} at ({}, {})", &target_rev, i, j);
        }
    }

    let height = lines.len();
    let width = lines[0].len();

    // main diagonal instances
    let n = target.len();
    for i in 0..=(height - n) {
        for j in 0..=(width - n) {
            let mut found = true;
            let mut found_rev = true;

            for k in 0..n {
                let t_char = target.chars().nth(k).unwrap();
                let t_rev_char = target_rev.chars().nth(k).unwrap();
                let c = lines[i + k].chars().nth(j + k).unwrap();

                if c != t_char {
                    found = false;
                }
                if c != t_rev_char {
                    found_rev = false;
                } else if !found && !found_rev {
                    break;
                }
            }

            if found {
                println!("found {} at ({}, {}) [DIAGONAL]", target, i, j);
                count += 1;
            }

            if found_rev {
                println!("found {} at ({}, {}) [DIAGONAL]", &target_rev, i, j);
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX"#;
        assert_eq!(part1(input), 18);
    }

    #[test]
    fn no_difference_between_inputs() {
        let input_1 = r#"....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX"#;

        let input_simple = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        assert_eq!(part1(input_1), part1(input_simple));
    }

    #[test]
    fn counts_all_directions() {
        let input: &str = r#"XMAS..
MM....
A.A...
S..S..
......
..AX.."#;
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn counts_all_directions_2() {
        let input: &str = r#"SAMX..
AA....
M.M...
X..X.X
.....X
.....X"#;
        assert_eq!(part1(input), 3);
    }
}
