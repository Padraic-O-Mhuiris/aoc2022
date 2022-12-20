use std::{
    collections::{HashMap, HashSet},
    str::Chars,
};

use itertools::Itertools;

fn q1() {
    let mut priorities: HashMap<char, u32> = HashMap::new();
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();

    for (i, c) in alphabet.chars().enumerate() {
        priorities.insert(c, u32::try_from(i + 1).unwrap());
    }

    let answer: u32 = include_str!("input.txt")
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(comp1, comp2)| {
            let c = *HashSet::intersection(
                &String::from(comp1).chars().collect::<HashSet<char>>(),
                &String::from(comp2).chars().collect::<HashSet<char>>(),
            )
            .nth(0)
            .unwrap();
            priorities.get(&c).unwrap()
        })
        .sum();

    println!("{answer:?}");
}

fn q2() {
    let mut priorities: HashMap<char, u32> = HashMap::new();
    let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();

    for (i, c) in alphabet.chars().enumerate() {
        priorities.insert(c, u32::try_from(i + 1).unwrap());
    }

    let answer: u32 = include_str!("input.txt")
        .lines()
        .chunks(3)
        .into_iter()
        .map(|v| {
            let batch = v.collect::<Vec<&str>>();

            let mut x: HashSet<char> = batch[0].chars().collect();
            let y: HashSet<char> = batch[1].chars().collect();
            let z: HashSet<char> = batch[2].chars().collect();

            x.retain(|item| y.contains(item) && z.contains(item));
            let c = x.into_iter().nth(0).unwrap();
            priorities.get(&c).unwrap()
        })
        .sum();

    println!("{answer:?}");
}

fn main() {
    q2()
}
