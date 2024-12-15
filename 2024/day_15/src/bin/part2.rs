use std::{char, vec};

fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(input: &str) -> usize {
    let input_parts = input.split("\n\n").collect::<Vec<&str>>();
    let map_raw = input_parts[0];
    let instructions_raw = input_parts[1];

    let mut map: Vec<Vec<char>> = map_raw
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    map = transform_map(map);

    let instructions = instructions_raw
        .chars()
        .filter(|&c| c != '\n')
        .collect::<Vec<char>>();

    let mut pos = (0, 0);
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '@' {
                pos = (x, y);
            }
        }
    }

    for row in map.iter() {
        println!("{}", row.iter().collect::<String>());
    }

    for instruction in instructions {
        push(&mut map, &mut pos, instruction);

        // println!("Instruction: {}", instruction);
        // for row in map.iter() {
        //     println!("{}", row.iter().collect::<String>());
        // }
    }

    for row in map.iter() {
        println!("{}", row.iter().collect::<String>());
    }

    let mut gps_coords_sum = 0;

    for (y, row) in map.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if cell == '[' {
                gps_coords_sum += x + y * 100;
            }
        }
    }
    gps_coords_sum
}

fn transform_map(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_map: Vec<Vec<char>> = vec![];

    for row in map {
        let mut new_row: Vec<char> = vec![];
        for cell in row {
            match cell {
                '#' => "##",
                '.' => "..",
                'O' => "[]",
                '@' => "@.",
                _ => panic!("Invalid cell"),
            }
            .chars()
            .for_each(|c| new_row.push(c));
        }
        new_map.push(new_row);
    }
    new_map
}

fn push(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize), instruction: char) {
    let direction = match instruction {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!("Invalid instruction"),
    };

    let target_pos: (usize, usize) = (
        pos.0.wrapping_add_signed(direction.0),
        pos.1.wrapping_add_signed(direction.1),
    );

    if can_move_into(map, target_pos, direction) {
        move_recursive(map, *pos, direction);
        map[pos.1][pos.0] = '.';
        *pos = target_pos;
        map[pos.1][pos.0] = '@';
    }
}

fn move_recursive(map: &mut Vec<Vec<char>>, pos: (usize, usize), direction: (isize, isize)) {
    let target_pos: (usize, usize) = (
        pos.0.wrapping_add_signed(direction.0),
        pos.1.wrapping_add_signed(direction.1),
    );

    let pos_c = map[pos.1][pos.0];
    let target_c = map[target_pos.1][target_pos.0];

    if target_c == '.' {
        map[target_pos.1][target_pos.0] = pos_c;
        return;
    }

    let horizontal = direction.0 != 0;
    if horizontal {
        move_recursive(map, target_pos, direction);
        map[target_pos.1][target_pos.0] = pos_c;
    } else {
        if target_c == '[' {
            // moves left part of crate
            move_recursive(map, target_pos, direction);
            map[target_pos.1][target_pos.0] = pos_c;

            // moves right part of crate

            move_recursive(map, (target_pos.0 + 1, target_pos.1), direction);
            map[target_pos.1][target_pos.0 + 1] = '.';
        } else if target_c == ']' {
            // moves right part of crate
            move_recursive(map, target_pos, direction);
            map[target_pos.1][target_pos.0] = pos_c;

            // moves left part of crate
            move_recursive(map, (target_pos.0 - 1, target_pos.1), direction);
            map[target_pos.1][target_pos.0 - 1] = '.';
        } else {
            panic!("Invalid cell {}", target_c);
        }
    }
}

fn can_move_into(
    map: &mut Vec<Vec<char>>,
    target_pos: (usize, usize),
    direction: (isize, isize),
) -> bool {
    let horizontal = direction.0 != 0;

    if horizontal {
        match map[target_pos.1][target_pos.0] {
            '#' => false,
            '.' => true,
            '[' | ']' => can_move_into(
                map,
                (target_pos.0.wrapping_add_signed(direction.0), target_pos.1),
                direction,
            ),

            _ => panic!("Invalid cell"),
        }
    } else {
        match map[target_pos.1][target_pos.0] {
            '#' => false,
            '.' => true,
            '[' => {
                can_move_into(
                    map,
                    (target_pos.0, target_pos.1.wrapping_add_signed(direction.1)),
                    direction,
                ) && can_move_into(
                    map,
                    (
                        target_pos.0 + 1,
                        target_pos.1.wrapping_add_signed(direction.1),
                    ),
                    direction,
                )
            }
            ']' => {
                can_move_into(
                    map,
                    (target_pos.0, target_pos.1.wrapping_add_signed(direction.1)),
                    direction,
                ) && can_move_into(
                    map,
                    (
                        target_pos.0 - 1,
                        target_pos.1.wrapping_add_signed(direction.1),
                    ),
                    direction,
                )
            }

            _ => panic!("Invalid cell"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;
        assert_eq!(part1(input), 9021);
    }

    #[test]
    fn it_follows_example_2() {
        let input: &str = r#"#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

vv<<<<^>^>^>>vv"#;
        part1(input);
    }
}
