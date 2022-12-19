fn q1() {
    let answer = include_str!("input.txt") // read from file
        .lines()
        .map(|v| match v.split_once(' ') {
            Some(("A", "X")) => 4, // Draw, Rock 3 + 1
            Some(("A", "Y")) => 8, // Win, Paper, 6 + 2
            Some(("A", "Z")) => 3, // Lose, Scissors, 0 + 3
            Some(("B", "X")) => 1, // Lose, Rock, 0 + 1
            Some(("B", "Y")) => 5, // Draw, Paper, 3 + 2
            Some(("B", "Z")) => 9, // Win, Scissors, 6 + 3
            Some(("C", "X")) => 7, // Win, Rock, 6 + 1
            Some(("C", "Y")) => 2, // Lose, Paper, 0 + 2
            Some(("C", "Z")) => 6, // Draw, Scissors, 3 + 3
            Some((_, _)) => 0,
            None => 0,
        })
        .sum::<u64>();

    println!("{answer:?}");
}

fn q2() {
    let answer = include_str!("input.txt") // read from file
        .lines()
        .map(|v| match v.split_once(' ') {
            Some(("A", "X")) => 3, // Lose, Scissors 0 + 3
            Some(("A", "Y")) => 4, // Draw, Rock, 3 + 1
            Some(("A", "Z")) => 8, // Win, Paper, 6 + 2
            Some(("B", "X")) => 1, // Lose, Rock, 0 + 1
            Some(("B", "Y")) => 5, // Draw, Paper, 3 + 2
            Some(("B", "Z")) => 9, // Win, Scissors, 6 + 3
            Some(("C", "X")) => 2, // Lose, Paper, 0 + 2
            Some(("C", "Y")) => 6, // Draw, Scissors, 3 + 3
            Some(("C", "Z")) => 7, // Win, Rock, 6 + 1
            Some((_, _)) => 0,
            None => 0,
        })
        .sum::<u64>();

    println!("{answer:?}");
}

fn main() {
    q2()
}
