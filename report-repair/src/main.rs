use std::collections::HashSet;

fn find_pair_that_sums_to(input: &str, target_sum: usize) -> (usize, usize) {
    let mut seen = HashSet::new();

    for line in input.lines() {
        let entry: usize = line.parse().unwrap();
        if entry < target_sum {
            let compliment = target_sum - entry;

            if seen.contains(&entry) {
                return (entry, compliment);
            }

            seen.insert(compliment);
        }
    }

    panic!("unable to find two numbers that sum to {}", target_sum);
}

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let (a, b) = find_pair_that_sums_to(&INPUT, 2020);
    println!("{} * {} = {}", a, b, a * b);
}
