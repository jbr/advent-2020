use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let parser = Regex::new(r"(\d+)-(\d+) (.): (.+)\n?").unwrap();
    let valid_rows = parser.captures_iter(INPUT).filter(|row| {
        let min = row[1].parse::<usize>().unwrap();
        let max = row[2].parse::<usize>().unwrap();
        let pat: &str = &row[3];
        let pass: &str = &row[4];
        let count = pass.matches(pat).count();
        (min..=max).contains(&count)
    });

    println!("valid rows: {}", valid_rows.count());
}
