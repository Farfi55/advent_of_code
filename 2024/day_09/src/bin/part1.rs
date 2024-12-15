fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    let mut checksum = 0;

    let disk_map: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut pos = 0;

    let mut left_id = 0;
    let mut left_idx = 0;

    let mut right_id = (disk_map.len() + 1) / 2;
    let mut right_idx = disk_map.len();
    if disk_map.len() % 2 == 1 {
        right_idx += 1;
    }

    let mut remaining_right: u32 = 0;

    loop {
        if left_id == right_id {
            for _ in 0..remaining_right {
                print!("{}", right_id);
                checksum += pos * right_id;
                pos += 1;
            }

            println!("\n\nleft_id == right_id ({})", left_id);
            break;
        }

        let x = disk_map[left_idx];

        if left_idx % 2 == 0 {
            // take from left
            for _ in 0..x {
                print!("{}", left_id);
                checksum += pos * left_id;
                pos += 1;
            }
            left_id += 1;
        } else {
            // take from right
            for _ in 0..x {
                while remaining_right == 0 {
                    right_idx -= 2;
                    right_id -= 1;
                    remaining_right = disk_map[right_idx];
                }
                print!("{}", right_id);
                checksum += pos * right_id;
                pos += 1;
                remaining_right -= 1;
            }
        }

        left_idx += 1;
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"2333133121414131402"#;
        assert_eq!(part1(input), 1928);
    }
}
