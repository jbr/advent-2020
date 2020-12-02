const INPUT: &str = include_str!("../input.txt");
const TARGET: usize = 2020;

fn main() {
    let mut compliments = [None; TARGET];
    let mut seen_numbers = Vec::new();

    for line in INPUT.lines() {
        let number = line.parse::<usize>().unwrap();

        if number > TARGET {
            continue;
        };

        if let Some(product) = compliments[number] {
            println!("{}", product * number);
            return;
        }

        for other_number in &seen_numbers {
            let sum = number + other_number;
            if sum < TARGET {
                compliments[TARGET - sum] = Some(number * other_number);
            }
        }

        seen_numbers.push(number);
    }
}
