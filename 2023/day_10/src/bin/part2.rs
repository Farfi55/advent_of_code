use std::{time::Instant, vec};

const NOT_EXPLORED: usize = 0;

fn main() {
    let now = Instant::now();

    let input = include_str!("./input.txt");
    let result = part2(input);
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

fn explore_pipes(pipes: &Vec<Vec<char>>, start_pos: (usize, usize)) -> (Vec<Vec<usize>>, usize) {
    let mut explored_pipes: Vec<Vec<usize>> = vec![vec![NOT_EXPLORED; pipes[0].len()]; pipes.len()];

    let mut dist = 10;
    explored_pipes[start_pos.0][start_pos.1] = dist;

    let first_conn = get_connected_pipes_pos(pipes, &start_pos)
        .into_iter()
        .filter(|pipe_pos| does_pipe_connect_to(pipes, pipe_pos, &start_pos))
        .next()
        .unwrap();

    // we just use one of the pipe connection, and follow it to the end
    dist += 1;
    explored_pipes[first_conn.0][first_conn.1] = dist;
    let mut next_pipe: Option<(usize, usize)> = Some(first_conn);

    while next_pipe.is_some() {
        let pipe_pos = next_pipe.take().unwrap();
        let connected_pipes = get_connected_pipes_pos(pipes, &pipe_pos);

        for connected_pipe in connected_pipes {
            if explored_pipes[connected_pipe.0][connected_pipe.1] == NOT_EXPLORED {
                dist += 1;
                explored_pipes[connected_pipe.0][connected_pipe.1] = dist;
                next_pipe = Some(connected_pipe);
                break;
            }
        }
    }
    (explored_pipes, dist)
}

fn part2(input: &str) -> usize {
    let pipes = parse(input);
    let start_pos = get_starting_pos(&pipes);
    let (explored_pipes, dist) = explore_pipes(&pipes, start_pos);
    println!("\nExplored pipes:");
    for line in explored_pipes.iter() {
        for pipe in line.iter() {
            print!("{:4}", pipe);
        }
        println!();
    }
    get_inside_pipes_count(&explored_pipes, dist)
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let pipes = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    pipes
}

fn get_inside_pipes_count(explored_pipes: &Vec<Vec<usize>>, dist: usize) -> usize {
    let mut inside_pipes = 0;
    for (y, line) in explored_pipes.iter().enumerate() {
        let mut is_inside_loop = false;
        for (x, pipe) in line.iter().enumerate() {
            if *pipe == NOT_EXPLORED {
                if is_inside_loop {
                    inside_pipes += 1;
                }
                continue;
            }

            let bottom_pipe = match y {
                y if y == explored_pipes.len() - 1 => 0,
                _ => explored_pipes[y + 1][x],
            };

            if pipe.abs_diff(bottom_pipe) == 1 {
                is_inside_loop = !is_inside_loop;
            } else if *pipe == 10 && bottom_pipe == dist {
                is_inside_loop = !is_inside_loop;
            } else if *pipe == dist && bottom_pipe == 10 {
                is_inside_loop = !is_inside_loop;
            }
        }
    }

    inside_pipes
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
        let input: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;
        assert_eq!(part2(input), 4);
    }

    #[test]
    fn it_follows_example_2() {
        let input: &str = r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
"#;
        assert_eq!(part2(input), 4);
    }

    #[test]
    fn it_follows_example_3() {
        let input: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;
        assert_eq!(part2(input), 8);
    }

    #[test]
    fn it_follows_example_4() {
        let input: &str = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;
        assert_eq!(part2(input), 10);
    }
}
