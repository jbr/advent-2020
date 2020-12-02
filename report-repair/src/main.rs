use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");
const TARGET: usize = 2020;

fn main() {
    let mut compliments = HashMap::new();
    let mut seen_numbers = Vec::new();

    for line in INPUT.lines() {
        let number = line.parse::<usize>().unwrap();

        if let Some(product) = compliments.get(&number) {
            println!("{}", product * number);
            return;
        }

        for other_number in &seen_numbers {
            let sum = number + other_number;
            if sum < TARGET {
                compliments.insert(TARGET - sum, number * other_number);
            }
        }

        seen_numbers.push(number);
    }
}
