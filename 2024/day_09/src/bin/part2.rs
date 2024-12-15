fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);
}

fn part2(input: &str) -> usize {
    let disk_map: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut starting_blocks = vec![];

    let mut pos: usize = 0;
    for i in 0..disk_map.len() {
        let block_length = disk_map[i];

        if i % 2 == 0 {
            starting_blocks.push((pos, i / 2, block_length));
        }
        pos += block_length;
    }

    let mut blocks = starting_blocks.clone();
    print_blocks(&blocks);

    for (i, block) in starting_blocks.into_iter().rev().enumerate() {
        let (_, id, length) = block;

        for j in 0..blocks.len() - 1 {
            if blocks[j].1 == id {
                break;
            }

            let empty_space: usize = blocks[j + 1].0 - (blocks[j].0 + blocks[j].2);
            if empty_space >= length {
                // println!(
                //     "Inserting block id {} at {} up to {}, empty space was {}",
                //     id,
                //     blocks[j].0 + blocks[j].2,
                //     blocks[j].0 + blocks[j].2 + length,
                //     empty_space
                // );
                blocks.retain(|b| b.1 != id);
                blocks.insert(j + 1, (blocks[j].0 + blocks[j].2, id, length));
                // println!("{:?}", blocks);
                break;
            }
        }

        // print_blocks(&blocks);
    }

    let mut checksum = 0;

    for (mut pos, id, length) in blocks {
        for _ in 0..length {
            checksum += pos * id;
            pos += 1;
        }
    }

    checksum
}

fn print_blocks(blocks: &Vec<(usize, usize, usize)>) {
    let mut last_pos = 0;
    for (pos, id, length) in blocks {
        if pos < &last_pos {
            println!("\n({} - {}) for id {}", pos, last_pos, id);

            panic!("pos < last_pos");
        }
        print!("{}", '.'.to_string().repeat(*pos - last_pos));
        print!("{}", id.to_string().repeat(*length));
        last_pos = *pos + *length;
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"2333133121414131402"#;
        assert_eq!(part2(input), 2858);
    }
}
