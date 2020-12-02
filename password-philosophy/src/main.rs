use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let parser = Regex::new(r"(\d+)-(\d+) (.): (.+)\n?").unwrap();
    let valid_rows = parser.captures_iter(INPUT).filter(|row| {
        let first_index = row[1].parse::<usize>().unwrap() - 1;
        let second_index = row[2].parse::<usize>().unwrap() - 1;
        let pat: &str = &row[3];
        let pass: &str = &row[4];

        let first_match = &pass[first_index..first_index + pat.len()] == pat;
        let second_match = &pass[second_index..second_index + pat.len()] == pat;
        first_match ^ second_match
    });

    println!("valid rows: {}", valid_rows.count());
}
