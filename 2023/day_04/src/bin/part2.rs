fn main() {
    let input = include_str!("./input.txt");
    let result = part2(input);
    println!("RESULT:\n{}", result);
}

fn part2(input: &str) -> usize {
    get_total_cards(input).iter().sum()
}

fn get_total_cards(input: &str) -> Vec<usize> {
    let lines_count = input.lines().count();
    let mut cards_copies = vec![1; lines_count];
    for (line_n, line) in input.lines().enumerate() {
        let points = points_for_card(line);
        add_copies_to_vec(&mut cards_copies, line_n, points);
    }

    cards_copies
}

fn add_copies_to_vec(cards_copies: &mut Vec<usize>, curr: usize, points: usize) {
    let curr_copies = cards_copies[curr];
    for i in 1..=points {
        if curr + i >= cards_copies.len() {
            break;
        }
        cards_copies[curr + i] += curr_copies;
    }
}

fn points_for_card(card_line: &str) -> usize {
    let mut body_split = card_line
        .split(":")
        .nth(1)
        .expect("line should always have a body")
        .split("|");

    let my_numbers_raw = body_split.next().unwrap();
    let winning_numbers_raw = body_split.next().unwrap();

    let mut my_numbers = [false; 100];
    for number in my_numbers_raw.split_ascii_whitespace() {
        let number = number.parse::<usize>().unwrap();
        my_numbers[number] = true;
    }

    let mut score = 0;
    for number in winning_numbers_raw.split_ascii_whitespace() {
        let number = number.parse::<usize>().unwrap();
        if my_numbers[number] == true {
            score += 1
        }
    }
    return score;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

        let result = super::part2(input);
        assert_eq!(result, 30);
    }

    #[test]
    fn get_points_for_line() {
        let cards_score = vec![
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 4),
            ("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2),
            ("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2),
            ("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1),
            ("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0),
            ("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0),
        ];

        for (card, score) in cards_score {
            let result = super::points_for_card(card);
            dbg!(card);
            assert_eq!(result, score)
        }
    }
}
