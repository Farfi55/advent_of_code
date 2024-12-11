use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    usize,
};

fn main() {
    let input = include_str!("./input.txt");
    let result = part1(input);
    println!("RESULT:\n{}", result);
}

fn part1(_input: &str) -> usize {
    let map = parse_input(_input);
    let path = find_path((0, 0), (map.len() - 1, map[0].len() - 1), &map);
    let cost = path.iter().skip(1).map(|&(x, y)| map[x][y] as usize).sum();
    cost
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn h(pos: (usize, usize), goal: (usize, usize)) -> usize {
    let (y1, x1) = pos;
    let (y2, x2) = goal;
    ((y1 as i32 - y2 as i32).abs() + (x1 as i32 - x2 as i32).abs()) as usize;

    0
}

#[derive(Eq, PartialEq)]
struct State {
    g_score: usize,
    pos: (usize, usize),
    dir: (isize, isize),
    steps_in_dir: i8,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.g_score.cmp(&self.g_score)
    }
}

fn find_path(
    start: (usize, usize),
    goal: (usize, usize),
    map: &Vec<Vec<u8>>,
) -> Vec<(usize, usize)> {
    let mut open_set: BinaryHeap<State> = BinaryHeap::new();
    open_set.push(State {
        pos: start,
        g_score: 0,
        dir: (0, 0),
        steps_in_dir: 0,
    });

    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut g_score: HashMap<(usize, usize), usize> = HashMap::new();
    g_score.insert(start, 0);
    let mut f_score: HashMap<(usize, usize), usize> = HashMap::new();
    f_score.insert(start, h(start, goal));

    while !open_set.is_empty() {
        let state = open_set.pop().unwrap();
        let current = state.pos;

        if current == goal {
            return reconstruct_path(came_from, current);
        }

        for neighbor in neighbors(current, &map, &came_from) {
            let cost = map[neighbor.0][neighbor.1];
            let tentative_g_score = g_score.get(&current).unwrap() + cost as usize;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + h(neighbor, goal));
                if !open_set.iter().any(|x| x.pos == neighbor) {
                    open_set.push(State {
                        pos: neighbor,
                        g_score: tentative_g_score,
                    });
                }
            }
        }
    }

    panic!("No path found");
}

fn reconstruct_path(
    came_from: HashMap<(usize, usize), (usize, usize)>,
    current: (usize, usize),
) -> Vec<(usize, usize)> {
    reconstruct_path_up_to(&came_from, current, None)
}

fn reconstruct_path_up_to(
    came_from: &HashMap<(usize, usize), (usize, usize)>,
    current: (usize, usize),
    size_limit: Option<usize>,
) -> Vec<(usize, usize)> {
    let mut total_path = vec![current];
    let mut current = current;

    while came_from.contains_key(&current) {
        current = came_from.get(&current).unwrap().clone();
        total_path.push(current);
        if let Some(size_limit) = size_limit {
            if total_path.len() >= size_limit {
                break;
            }
        }
    }
    total_path
}

fn neighbors(
    pos: (usize, usize),
    map: &Vec<Vec<u8>>,
    came_from: &HashMap<(usize, usize), (usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];

    let dir = last_3_dirs_match(pos, &came_from);
    let (y, x) = pos;
    if y > 0 && dir != Some((-1, 0)) {
        neighbors.push((y - 1, x));
    }
    if x > 0 && dir != Some((0, -1)) {
        neighbors.push((y, x - 1));
    }
    if y < map.len() - 1 && dir != Some((1, 0)) {
        neighbors.push((y + 1, x));
    }
    if x < map[0].len() - 1 && dir != Some((0, 1)) {
        neighbors.push((y, x + 1));
    }
    neighbors
}

fn last_3_dirs_match(
    current: (usize, usize),
    came_from: &HashMap<(usize, usize), (usize, usize)>,
) -> Option<(isize, isize)> {
    let path = reconstruct_path_up_to(came_from, current, Some(4));
    if path.len() < 4 {
        return None;
    }

    let mut dir = (0, 0);
    for window in path.windows(2) {
        let (y2, x2) = window[0];
        let (y1, x1) = window[1];
        let (dy, dx) = ((y2 as isize - y1 as isize), (x2 as isize - x1 as isize));
        if dir == (0, 0) {
            dir = (dy, dx);
        } else if dir != (dy, dx) {
            return None;
        }
    }
    Some(dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_follows_example() {
        let input: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
        assert_eq!(part1(input), 102);
    }

    #[test]
    fn draw_path() {
        let input: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

        let map = parse_input(input);
        let path = find_path((0, 0), (12, 12), &map);
        let mut map: Vec<Vec<char>> = map
            .iter()
            .map(|line| line.iter().map(|&x| (x + 48) as char).collect())
            .collect();
        for (y, x) in path {
            map[y][x] = '.';
        }
        for line in map {
            println!("{:?}", line.iter().collect::<String>());
        }
    }

    #[test]
    fn test_last_3_dirs_match() {
        let mut came_from: HashMap<(usize, usize), (usize, usize)> =
            [((0, 1), (0, 0)), ((0, 2), (0, 1)), ((0, 3), (0, 2))]
                .iter()
                .cloned()
                .collect();

        assert_eq!(last_3_dirs_match((0, 3), &came_from), Some((0, 1)));

        came_from.insert((0, 4), (0, 3));

        assert_eq!(last_3_dirs_match((0, 4), &came_from), Some((0, 1)));

        came_from.insert((1, 4), (0, 4));

        assert_eq!(last_3_dirs_match((1, 4), &came_from), None);

        came_from.insert((2, 4), (1, 4));

        assert_eq!(last_3_dirs_match((2, 4), &came_from), None);

        came_from.insert((3, 4), (2, 4));

        assert_eq!(last_3_dirs_match((3, 4), &came_from), Some((1, 0)));
    }
}
