use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");
const TARGET: usize = 2020;

fn main() {
    let mut compliments_and_products: HashMap<usize, usize> = HashMap::new();
    let mut seen_numbers: Vec<usize> = Vec::with_capacity(200);

    for line in INPUT.lines() {
        let number = line.parse().unwrap();

        if let Some(product) = compliments_and_products.get(&number) {
            println!("{}", product * number);
            return;
        }

        for other_number in &seen_numbers {
            let sum = number + other_number;
            if sum < TARGET {
                compliments_and_products.insert(TARGET - sum, number * other_number);
            }
        }

        seen_numbers.push(number);
    }
}
