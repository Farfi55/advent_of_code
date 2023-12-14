use std::{collections::HashMap, time::Instant};

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

fn part2(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| {
            let mut record = parse_condition_record(line);
            record = expand_condition_recort(&record);
            get_possible_arrangements(&mut record)
        })
        .sum()
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum Condition {
    Working,
    Broken,
    Unknown,
}

impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Condition::Working => '.',
            Condition::Broken => '#',
            Condition::Unknown => '?',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct ConditionRecord {
    conditions: Vec<Condition>,
    broken_groups: Vec<usize>,
}

impl std::fmt::Display for ConditionRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let conditions = self
            .conditions
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("");
        let broken_groups = self
            .broken_groups
            .iter()
            .map(|g| g.to_string())
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{} {}", conditions, broken_groups)
    }
}

fn parse_condition_record(input: &str) -> ConditionRecord {
    let mut parts = input.split_whitespace();
    let conditions_raw = parts.next().unwrap();
    let broken_groups_raw = parts.next().unwrap();

    let conditions = conditions_raw
        .chars()
        .map(|c| match c {
            '#' => Condition::Broken,
            '.' => Condition::Working,
            '?' => Condition::Unknown,
            _ => panic!("Invalid condition character: {}", c),
        })
        .collect();

    let broken_groups = broken_groups_raw
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    ConditionRecord {
        conditions,
        broken_groups,
    }
}

fn expand_condition_recort(record: &ConditionRecord) -> ConditionRecord {
    let mut conditions: Vec<Condition> = Vec::with_capacity((record.conditions.len() + 4) * 5);
    let mut broken_groups: Vec<usize> = Vec::with_capacity(record.broken_groups.len() * 5);

    for i in 0..5 {
        for condition in record.conditions.iter() {
            conditions.push(condition.clone());
        }
        if i != 4 {
            conditions.push(Condition::Unknown);
        }
        for group in record.broken_groups.iter() {
            broken_groups.push(*group);
        }
    }

    ConditionRecord {
        conditions,
        broken_groups,
    }
}

fn get_possible_arrangements(record: &mut ConditionRecord) -> usize {
    let mut arrangements_per_config: HashMap<ConditionRecord, usize> = HashMap::new();
    recurse(record.clone(), &mut arrangements_per_config)
}

fn recurse(
    record: ConditionRecord,
    arrangements_per_config: &mut HashMap<ConditionRecord, usize>,
) -> usize {
    if arrangements_per_config.contains_key(&record) {
        return *arrangements_per_config.get(&record).unwrap();
    }
    let mut count = 0;

    let next_group = record.broken_groups.first();

    // BASE CASE:
    if next_group.is_none() {
        // if any of the remaining conditions are broken, this is not a valid arrangement
        count = if record.conditions.iter().any(|c| *c == Condition::Broken) {
            0
        } else {
            1
        };
        arrangements_per_config.insert(record.clone(), count);
        return count;
    }

    let next_group = *next_group.unwrap();

    let next_condition = record.conditions.first();
    if next_condition.is_none() {
        arrangements_per_config.insert(record.clone(), 0);
        return 0;
    }
    let next_condition = next_condition.unwrap();

    count = match next_condition {
        Condition::Working => handle_working(&record, arrangements_per_config),
        Condition::Broken => handle_broken(&record, next_group, arrangements_per_config),
        Condition::Unknown => {
            handle_working(&record, arrangements_per_config)
                + handle_broken(&record, next_group, arrangements_per_config)
        }
    };
    arrangements_per_config.insert(record.clone(), count);
    count
}

fn handle_working(
    record: &ConditionRecord,
    arrangements_per_config: &mut HashMap<ConditionRecord, usize>,
) -> usize {
    let new_record = ConditionRecord {
        conditions: record.conditions[1..].to_vec(),
        broken_groups: record.broken_groups.to_vec(),
    };
    recurse(new_record, arrangements_per_config)
}

fn handle_broken(
    record: &ConditionRecord,
    next_group: usize,
    arrangements_per_config: &mut HashMap<ConditionRecord, usize>,
) -> usize {
    let mut possibly_broken_count = 1;
    for (i, condition) in record.conditions.iter().skip(1).enumerate() {
        match condition {
            Condition::Working => break,
            Condition::Broken => possibly_broken_count += 1,
            Condition::Unknown => {
                if possibly_broken_count < next_group {
                    possibly_broken_count += 1;
                } else {
                    break;
                }
            }
        }
    }
    if possibly_broken_count == next_group {
        let conditions = if record.conditions.len() > next_group {
            record.conditions[next_group + 1..].to_vec()
        } else {
            vec![]
        };

        let new_record = ConditionRecord {
            conditions,
            broken_groups: record.broken_groups[1..].to_vec(),
        };
        recurse(new_record, arrangements_per_config)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn it_follows_example() {
        assert_eq!(part2(EXAMPLE_INPUT), 525152);
    }

    #[test]
    fn test_expand() {
        let cases = vec![
            (".# 1", ".#?.#?.#?.#?.# 1,1,1,1,1"),
            (
                "???.### 1,1,3",
                "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3",
            ),
        ];
        for (input, expected) in cases {
            let record = parse_condition_record(input);
            let expanded = expand_condition_recort(&record);
            assert_eq!(expanded.to_string(), expected);
        }
    }

    #[test]
    fn parse_condition_record_works() {
        let cases = vec![
            (
                "???.### 1,1,3",
                ConditionRecord {
                    conditions: vec![
                        Condition::Unknown,
                        Condition::Unknown,
                        Condition::Unknown,
                        Condition::Working,
                        Condition::Broken,
                        Condition::Broken,
                        Condition::Broken,
                    ],
                    broken_groups: vec![1, 1, 3],
                },
            ),
            (
                ".??..??...?##. 1,1,3",
                ConditionRecord {
                    conditions: vec![
                        Condition::Working,
                        Condition::Unknown,
                        Condition::Unknown,
                        Condition::Working,
                        Condition::Working,
                        Condition::Unknown,
                        Condition::Unknown,
                        Condition::Working,
                        Condition::Working,
                        Condition::Working,
                        Condition::Unknown,
                        Condition::Broken,
                        Condition::Broken,
                        Condition::Working,
                    ],
                    broken_groups: vec![1, 1, 3],
                },
            ),
        ];
        for (input, expected) in cases {
            let condition_record = parse_condition_record(input);
            assert_eq!(condition_record, expected);
        }
    }

    #[test]
    fn arrangements_count_is_correct() {
        let cases = vec![
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 16384),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 16),
            ("????.######..#####. 1,6,5", 2500),
            ("?###???????? 3,2,1", 506250),
        ];

        for (input, expected) in cases {
            let mut condition_record = parse_condition_record(input);
            condition_record = expand_condition_recort(&condition_record);
            assert_eq!(get_possible_arrangements(&mut condition_record), expected);
        }
    }
}
