fn main() {
    println!(
        "trees: {}",
        include_bytes!("../input.txt")
            .split(|c| c == &b'\n')
            .filter(|line| line.len() > 0)
            .enumerate()
            .filter(|(line_number, line)| line[(line_number * 3) % line.len()] == b'#')
            .count()
    );
}
