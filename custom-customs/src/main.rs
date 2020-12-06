#![feature(iterator_fold_self)]

use std::collections::HashSet;

fn main() {
    main_part_2()
}

#[allow(dead_code)]
fn main_part_1() {
    println!(
        "{}",
        sum_counts_from_groups(include_str!("../input.txt"), unique_questions_per_group)
    );
}

fn main_part_2() {
    println!(
        "{}",
        sum_counts_from_groups(include_str!("../input.txt"), questions_everyone_answered)
    );
}

#[allow(dead_code)]
fn unique_questions_per_group(group: &str) -> usize {
    group
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<_>>()
        .len()
}

#[cfg(test)]
#[test]
fn test_unique_questions_per_group() {
    assert_eq!(unique_questions_per_group("abc"), 3);
    assert_eq!(unique_questions_per_group("a\nb\nc"), 3);
    assert_eq!(unique_questions_per_group("ab\nac"), 3);
    assert_eq!(unique_questions_per_group("a\na\na\na"), 1);
    assert_eq!(unique_questions_per_group("b"), 1);
}

fn questions_everyone_answered(group: &str) -> usize {
    group
        .lines()
        .map(|person| person.chars().collect::<HashSet<_>>())
        .fold_first(|x, y| x.intersection(&y).map(|c| *c).collect::<HashSet<_>>())
        .unwrap()
        .len()
}

#[cfg(test)]
#[test]
fn test_questions_everyone_answered() {
    assert_eq!(questions_everyone_answered("abc"), 3);
    assert_eq!(questions_everyone_answered("a\nb\nc"), 0);
    assert_eq!(questions_everyone_answered("ab\nac"), 1);
    assert_eq!(questions_everyone_answered("a\na\na\na"), 1);
    assert_eq!(questions_everyone_answered("b"), 1);
}

fn sum_counts_from_groups<F>(input: &str, f: F) -> usize
where
    F: Fn(&str) -> usize,
{
    input.split("\n\n").map(f).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

    #[test]
    fn test_sum_counts_from_groups_with_unique_questions_per_group() {
        assert_eq!(
            sum_counts_from_groups(TEST_INPUT, unique_questions_per_group),
            11
        );
    }

    #[test]
    fn test_sum_counts_from_groups_with_questions_everybody_answered() {
        assert_eq!(
            sum_counts_from_groups(TEST_INPUT, questions_everyone_answered),
            6
        );
    }
}
