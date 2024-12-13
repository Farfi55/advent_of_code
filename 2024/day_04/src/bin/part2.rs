use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);
}

fn part2(input: &str) -> usize {
    let mut count: usize = 0;
    let mut lines: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    let height = lines.len();
    let width = lines[0].len();

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let c = lines[i][j];
            if c != 'A' {
                println!("skipping ({}, {}): {}", i, j, c);
                continue;
            }

            let tl = lines[i - 1][j - 1];
            let tr = lines[i - 1][j + 1];
            let bl = lines[i + 1][j - 1];
            let br = lines[i + 1][j + 1];

            let corners = &vec![tl, tr, bl, br];
            if corners.iter().filter(|&&c| c == 'M').count() != 2 {
                println!(
                    "skipping, not enough M corners ({}, {}): {:?}",
                    i, j, corners
                );
                continue;
            }
            if corners.iter().filter(|&&c| c == 'S').count() != 2 {
                println!(
                    "skipping, not enough X corners ({}, {}): {:?}",
                    i, j, corners
                );
                continue;
            }

            if tl == br || tr == bl {
                println!("skipping, corners matcth ({}, {}): {:?}", i, j, corners);
                continue;
            }

            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"M.S
.A.
M.S"#;
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn it_follows_example_2() {
        let input: &str = r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#;
        assert_eq!(part2(input), 9);
    }
}
