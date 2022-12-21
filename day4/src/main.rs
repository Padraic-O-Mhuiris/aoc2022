fn parse_elf_section(elf: &str) -> (u32, u32) {
    (match elf.split_once("-") {
        Some((e1, e2)) => Some((e1.parse::<u32>().unwrap(), e2.parse::<u32>().unwrap())),
        None => None,
    })
    .unwrap()
}

fn q1() {
    let answer: u32 = include_str!("input.txt")
        .lines()
        .map(|l| {
            let (elf1, elf2) = l.split_once(",").unwrap();
            let (elf1_start, elf1_finish) = parse_elf_section(elf1);
            let (elf2_start, elf2_finish) = parse_elf_section(elf2);

            if (elf1_start <= elf2_start && elf1_finish >= elf2_finish)
                || (elf2_start <= elf1_start && elf2_finish >= elf1_finish)
            {
                return 1;
            }
            0
        })
        .sum();
    println!("{answer:?}");
}

fn q2() {
    let answer: u32 = include_str!("input.txt")
        .lines()
        .map(|l| {
            let (elf1, elf2) = l.split_once(",").unwrap();
            let (elf1_start, elf1_finish) = parse_elf_section(elf1);
            let (elf2_start, elf2_finish) = parse_elf_section(elf2);

            if (elf1_start <= elf2_start && elf1_finish >= elf2_start)
                || (elf1_start <= elf2_finish && elf1_finish >= elf2_finish)
                || (elf2_start <= elf1_start && elf2_finish >= elf1_start)
                || (elf2_start <= elf1_finish && elf2_finish >= elf1_finish)
            {
                return 1;
            }
            0
        })
        .sum();
    println!("{answer:?}");
}

fn main() {
    q2();
}
