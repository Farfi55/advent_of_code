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

    for instruction in instructions {
        push(&mut map, &mut pos, instruction);

        // Uncomment to see the map after each instruction
        // for row in map.iter() {
        //     println!("{}", row.iter().collect::<String>());
        // }
    }

    let mut gps_coords_sum = 0;

    for (y, row) in map.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if cell == 'O' {
                gps_coords_sum += x + y * 100;
            }
        }
    }
    gps_coords_sum
}

fn push(map: &mut Vec<Vec<char>>, pos: &mut (usize, usize), instruction: char) {
    let direction = match instruction {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!("Invalid instruction"),
    };

    let mut target_pos: (usize, usize) = (
        pos.0.wrapping_add_signed(direction.0),
        pos.1.wrapping_add_signed(direction.1),
    );

    if !can_move_into(map, target_pos, direction) {
        return;
    }

    map[pos.1][pos.0] = '.';
    *pos = target_pos;

    let mut c = '@';
    let mut target_c = map[target_pos.1][target_pos.0];

    while map[target_pos.1][target_pos.0] == 'O' {
        map[target_pos.1][target_pos.0] = c;
        c = target_c;
        target_pos = (
            target_pos.0.wrapping_add_signed(direction.0),
            target_pos.1.wrapping_add_signed(direction.1),
        );
        target_c = map[target_pos.1][target_pos.0];
    }
    map[target_pos.1][target_pos.0] = c;
}

fn can_move_into(
    map: &mut Vec<Vec<char>>,
    target_pos: (usize, usize),
    direction: (isize, isize),
) -> bool {
    match map[target_pos.1][target_pos.0] {
        '#' => false,
        '.' => true,
        'O' => can_move_into(
            map,
            (
                target_pos.0.wrapping_add_signed(direction.0),
                target_pos.1.wrapping_add_signed(direction.1),
            ),
            direction,
        ),
        _ => panic!("Invalid cell"),
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
        assert_eq!(part1(input), 10092);
    }

    #[test]
    fn it_follows_example2() {
        let input: &str = r#"########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"#;
        assert_eq!(part1(input), 2028);
    }
}
