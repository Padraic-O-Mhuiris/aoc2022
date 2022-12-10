use std::{cmp::Reverse, fs};

use itertools::Itertools;

#[derive(Clone, Copy)]
struct Elf {
    idx: i32,
    calories: u128,
}

fn solution() {
    let data = fs::read_to_string("src/input.txt").expect("Unable to read file");

    let mut sum = 0;
    let mut elf_idx = 1;

    let mut elves: Vec<Elf> = [
        Elf {
            idx: 0,
            calories: 0,
        },
        Elf {
            idx: 0,
            calories: 0,
        },
        Elf {
            idx: 0,
            calories: 0,
        },
    ]
    .to_vec();

    for n in data.split('\n') {
        match n.parse::<u128>() {
            Ok(amount) => sum += amount,
            Err(_) => {
                let elf = Elf {
                    idx: elf_idx,
                    calories: sum,
                };

                if elf.calories > elves[0].calories {
                    elves = [elf, elves[0], elves[1]].to_vec()
                } else if elf.calories > elves[1].calories {
                    elves = [elves[0], elf, elves[1]].to_vec()
                } else if elf.calories > elves[2].calories {
                    elves = [elves[0], elves[1], elf].to_vec()
                }
                elf_idx += 1;
                sum = 0;
            }
        }
    }

    let mut total = 0;
    for elf in elves {
        println!("ELF: {}", elf.idx);
        println!("CALORIES: {}", elf.calories);
        total += elf.calories
    }

    println!("TOTAL_CALORIES: {}", total)
}

fn faster_than_lime() {
    let answer = include_str!("input.txt") // read from file
        .lines() // Break into lines
        .map(|v| v.parse::<u64>().ok()) // while iterating, parse line as u64 and .ok() to change result into an option
        .batching(|mut it| (&mut it).map_while(|x| x).sum1::<u64>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();
    println!("{answer:?}");
}

fn main() {
    // solution()
    faster_than_lime();
}
