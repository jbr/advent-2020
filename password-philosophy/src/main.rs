use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

struct Row<'a> {
    first_index: usize,
    second_index: usize,
    pattern: &'a str,
    password: &'a str,
}

lazy_static! {
    static ref PARSER: Regex = Regex::new(r"(\d+)-(\d+) (.): (.+)\n?").unwrap();
}

impl<'a> Row<'a> {
    fn all(input: &'a str) -> impl Iterator<Item = Row<'a>> {
        PARSER.captures_iter(input).map(|captures| Row {
            first_index: captures[1].parse::<usize>().unwrap() - 1,
            second_index: captures[2].parse::<usize>().unwrap() - 1,
            pattern: captures.get(3).unwrap().as_str(),
            password: captures.get(4).unwrap().as_str(),
        })
    }

    fn is_match(&self, index: usize) -> bool {
        &self.password[index..index + self.pattern.len()] == self.pattern
    }

    fn first_match(&self) -> bool {
        self.is_match(self.first_index)
    }

    fn second_match(&self) -> bool {
        self.is_match(self.second_index)
    }

    fn is_valid(&self) -> bool {
        self.first_match() ^ self.second_match()
    }
}

fn main() {
    println!(
        "valid rows: {}",
        Row::all(INPUT).filter(Row::is_valid).count()
    );
}
