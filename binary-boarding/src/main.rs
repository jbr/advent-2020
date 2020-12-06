use std::{ops::Range, str::FromStr};

#[derive(Debug, Eq, PartialEq)]
struct BoardingPass {
    row: usize,
    column: usize,
}
type Result<T> = std::result::Result<T, &'static str>;

fn narrow(input: &str, range: Range<usize>, upper: char, lower: char) -> Result<usize> {
    let range = input.chars().fold(range, |range, c| {
        let half = range.start + (range.end - range.start) / 2;
        if c == lower {
            range.start..half
        } else if c == upper {
            half..range.end
        } else {
            panic!("unexpected character");
        }
    });

    assert_eq!(range.start, range.end - 1);
    Ok(range.start)
}

impl BoardingPass {
    fn parse_row(s: &str) -> Result<usize> {
        narrow(s, 0..128, 'B', 'F')
    }

    fn parse_column(s: &str) -> Result<usize> {
        narrow(s, 0..8, 'R', 'L')
    }

    fn seat_id(&self) -> usize {
        self.row * 8 + self.column
    }

    fn parse(s: &str) -> Result<Self> {
        Ok(Self {
            row: Self::parse_row(&s[0..7])?,
            column: Self::parse_column(&s[7..])?,
        })
    }
}

impl FromStr for BoardingPass {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parsing() -> Result<()> {
        let pass: BoardingPass = "FBFBBFFRLR".parse()?;
        assert_eq!(pass.row, 44);
        assert_eq!(pass.column, 5);
        assert_eq!(pass.seat_id(), 357);

        let pass: BoardingPass = "BFFFBBFRRR".parse()?;
        assert_eq!(pass.row, 70);
        assert_eq!(pass.column, 7);
        assert_eq!(pass.seat_id(), 567);

        let pass: BoardingPass = "FFFBBBFRRR".parse()?;
        assert_eq!(pass.row, 14);
        assert_eq!(pass.column, 7);
        assert_eq!(pass.seat_id(), 119);

        let pass: BoardingPass = "BBFFBBFRLL".parse()?;
        assert_eq!(pass.row, 102);
        assert_eq!(pass.column, 4);
        assert_eq!(pass.seat_id(), 820);

        Ok(())
    }
}

#[allow(dead_code)]
fn main_part_1() {
    println!(
        "max seat id: {}",
        include_str!("../input.txt")
            .lines()
            .map(|boarding_pass| BoardingPass::parse(boarding_pass).unwrap().seat_id())
            .max()
            .unwrap()
    );
}

fn main_part_2() {
    let mut boarding_passes = include_str!("../input.txt")
        .lines()
        .map(|boarding_pass| BoardingPass::parse(boarding_pass).unwrap().seat_id())
        .collect::<Vec<_>>();
    boarding_passes.sort();
    let missing_boarding_pass = boarding_passes
        .windows(2)
        .find_map(|x| {
            if x[0] == x[1] - 1 {
                None
            } else {
                Some(x[0] + 1)
            }
        })
        .unwrap();

    println!("empty seat {}", missing_boarding_pass);
}

fn main() {
    main_part_2()
}
