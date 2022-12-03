fn priority(val: char) -> u32 {
    match val {
        'a'..='z' => val as u32 - 'a' as u32 + 1,
        'A'..='Z' => val as u32 - 'A' as u32 + 27,
        _ => panic!("Invalid character {}", val),
    }
}

fn main() {
    let lines = include_str!("../../../input03")
        .lines().collect::<Vec<&str>>();
    
    println!("Part1 sum of priority for shared items: {}", lines.iter()
        .map(|l| l.trim().split_at(l.len()/2))
        .map(|(r1, r2)| r1.chars()
            .find(|c| r2.chars().any(|c1| &c1 == c))    // One char from the first must be in the other
            .expect("Must have shared item"))
        .map(priority)
        .sum::<u32>());

    println!("Part2: {}", (0..lines.len())
        .step_by(3)
        .map(|i| lines[i].chars()
            .find(|c| lines[i + 1].chars().any(|c1| &c1 == c) && lines[i + 2].chars().any(|c2| &c2 == c))   // One char from the first line must be in both of the others
            .expect("Must have common type"))
        .map(priority)
        .sum::<u32>());
}
