use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");
    let result = part2(input);
    println!("sum:\n{}", result);
}

fn part2(input: &str) -> u32 {
    // map of string to i32, where the string is a number and the i32 is the number of times it appears
    let mut map: HashMap<&str, u32> = std::collections::HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);

    let mut sum = 0;
    for line in input.lines() {
        // create a list of numbers in the line
        let mut line_digits: Vec<u32> = Vec::new();
        for i in 0..line.chars().count() {
            let digit = line.chars().nth(i).unwrap();
            if digit.is_numeric() {
                line_digits.push(digit.to_digit(10).unwrap());
            } else {
                for key in map.keys() {
                    if line[i..].starts_with(key) {
                        line_digits.push(map.get(key).unwrap().clone());
                        break;
                    }
                }
            }
        }
        // assemble the final number using the first and last digits of
        let num = line_digits.first().unwrap() * 10 + line_digits.last().unwrap();
        print!("{} ", num);
        sum += num;
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

        let result = super::part2(input);
        assert_eq!(result, 281);
    }
}
