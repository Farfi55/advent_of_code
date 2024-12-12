fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);
}

fn part2(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        let levels: Vec<isize> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if is_safe(&levels, true) {
            result += 1;
            print!("SAFE: ");
        } else {
            print!("UNSAFE: ");
        }
        println!("{:?}", levels);
    }
    result
}

fn is_safe(levels: &Vec<isize>, allow_remove: bool) -> bool {
    let mut growth = None;

    for i in 1..levels.len() {
        let diff = levels[i - 1] - levels[i];
        if growth.is_none() {
            growth = Some(diff.signum())
        }

        if growth.unwrap() != diff.signum() || diff.abs() > 3 || diff == 0 {
            if !allow_remove {
                return false;
            }

            let mut copy_1 = levels.clone();
            copy_1.remove(i - 1);
            if is_safe(&copy_1, false) {
                print!("BY REMOVING {} ", levels[i - 1]);
                return true;
            }

            let mut copy_2 = levels.clone();
            copy_2.remove(i);
            if is_safe(&copy_2, false) {
                print!("BY REMOVING {} ", levels[i]);
                return true;
            }

            if i >= 2 {
                let mut copy_3 = levels.clone();
                copy_3.remove(i - 2);
                if is_safe(&copy_3, false) {
                    print!("------- BY REMOVING {} ", levels[i - 2]);
                    return true;
                }
            }

            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;
        assert_eq!(part2(input), 4);
    }
}
