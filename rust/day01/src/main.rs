fn main() {
    let lines = include_str!("../../../input01").lines();

    let mut elfs = Vec::new();

    let mut current_sum = 0;
    for line in lines {
        if line.trim().is_empty() {
            elfs.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += line.parse::<u32>().unwrap();
        }
    }

    elfs.sort();

    println!("Elfs: {}", elfs.len());
    println!("Part1 max elf: {:?}", elfs.last());
    println!("Part2 top three sum elfs: {:?}", elfs.iter().rev().take(3).sum::<u32>());
}
