fn main() {
    let input = include_bytes!("../input.txt");
    let product_of_trees: usize = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(x, y)| check_slope(input, *x, *y))
        .product();

    println!("product of encountered trees: {}", product_of_trees);
}

fn check_slope(input: &[u8], x: usize, y: usize) -> usize {
    input
        .split(|c| *c == b'\n')
        .filter(|line| !line.is_empty())
        .step_by(y)
        .enumerate()
        .filter(|(line_number, line)| line[(*line_number * x) % line.len()] == b'#')
        .count()
}
