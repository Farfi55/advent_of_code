use std::{collections::VecDeque, time::Instant, vec};

fn main() {
    let now = Instant::now();

    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);

    let elapsed_time = now.elapsed();
    print_elapsed_time(elapsed_time);
}

fn print_elapsed_time(elapsed_time: std::time::Duration) {
    if elapsed_time.as_secs() == 0 {
        println!(
            "solving the problem took {} milliseconds.",
            elapsed_time.as_millis()
        );
    } else {
        println!(
            "solving the problem took {} seconds.",
            elapsed_time.as_secs()
        );
    }
}

fn does_pipe_connect_to(
    pipes: &Vec<Vec<char>>,
    pipe_pos: &(usize, usize),
    connection_pos: &(usize, usize),
) -> bool {
    let pipe_connections = get_connected_pipes_pos(pipes, pipe_pos);
    pipe_connections.contains(&connection_pos)
}

fn get_pipe_connections(pipe: &char) -> Vec<(isize, isize)> {
    match pipe {
        '|' => vec![(1, 0), (-1, 0)],
        '-' => vec![(0, 1), (0, -1)],
        'L' => vec![(-1, 0), (0, 1)],
        'J' => vec![(-1, 0), (0, -1)],
        '7' => vec![(1, 0), (0, -1)],
        'F' => vec![(1, 0), (0, 1)],
        '.' => vec![],
        'S' => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
        _ => panic!("Unknown pipe: {}", pipe),
    }
}

fn get_connected_pipes_pos(pipes: &Vec<Vec<char>>, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let pipes_size = (pipes.len(), pipes[0].len());

    let pipe_conn = get_pipe_connections(&pipes[pos.0][pos.1])
        .into_iter()
        .filter_map(|(y, x)| {
            let new_pos = (pos.0 as isize + y, pos.1 as isize + x);
            if new_pos.0 >= 0
                && new_pos.0 < pipes_size.0 as isize
                && new_pos.1 >= 0
                && new_pos.1 < pipes_size.1 as isize
            {
                Some((new_pos.0 as usize, new_pos.1 as usize))
            } else {
                None
            }
        })
        .collect::<Vec<(usize, usize)>>();

    pipe_conn
}

struct PipeExploration {
    last_pos: (usize, usize),
    next_pos: (usize, usize),
    next_distance: usize,
}

fn explore_pipes(pipes: &Vec<Vec<char>>, start_pos: (usize, usize)) -> usize {
    let mut next_pipes: VecDeque<PipeExploration> = VecDeque::new();
    let mut max_distance = 0;
    let mut reached_end = false;
    let starting_connections = get_connected_pipes_pos(pipes, &start_pos)
        .into_iter()
        .filter(|pos| does_pipe_connect_to(pipes, &pos, &start_pos));

    for pos in starting_connections {
        next_pipes.push_back(PipeExploration {
            last_pos: start_pos,
            next_pos: pos,
            next_distance: 1,
        });
    }

    while !next_pipes.is_empty() {
        let pipe_exploration = next_pipes.pop_front().unwrap();
        let pos = pipe_exploration.next_pos;
        let distance = pipe_exploration.next_distance;
        max_distance = std::cmp::max(max_distance, distance);

        let next_pipe = get_connected_pipes_pos(pipes, &pos)
            .into_iter()
            .filter(|pos| pos != &pipe_exploration.last_pos)
            .next();

        let next_pipe = match next_pipe {
            Some(p) => p,
            None => continue,
        };

        if reached_end || next_pipes.iter().any(|p| p.next_pos == next_pipe) {
            reached_end = true;
            continue;
        }
        next_pipes.push_back(PipeExploration {
            last_pos: pos,
            next_pos: next_pipe,
            next_distance: distance + 1,
        });
    }
    max_distance
}

fn part1(input: &str) -> usize {
    let pipes = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let start_pos = get_starting_pos(&pipes);
    explore_pipes(&pipes, start_pos)
}

fn get_starting_pos(pipes: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, line) in pipes.iter().enumerate() {
        for (x, pipe) in line.iter().enumerate() {
            if *pipe == 'S' {
                return (y, x);
            }
        }
    }
    panic!("No starting position found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example_1() {
        let input: &str = r#".....
.S-7.
.|.|.
.L-J.
.....
"#;
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn it_follows_example_2() {
        let input: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
        assert_eq!(part1(input), 8);
    }
}
