use std::fs;

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

fn faster_than_lime() -> color_eyre::Result<()> {
    color_eyre::install()?; // error report handler

    let lines = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let max = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .max();

    println!("{max:?}");
    Ok(())
}

fn main() {
    // solution()
    faster_than_lime();
}
