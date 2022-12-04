fn main() {
    let pairs = include_str!("../../../input04").lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(['-',','])
            .map(|v| v.parse::<u32>().expect(&format!("{} should be valid number", v)))
            .collect::<Vec<u32>>())
        .map(|nums| ((nums[0], nums[1]), (nums[2], nums[3])))
        .collect::<Vec<((u32, u32), (u32, u32))>>();

    println!("Part1: {}", pairs.iter()
        .filter(|(p1, p2)| is_within(p1, p2) || is_within(p2, p1))
        .count()
    );

    println!("Part2: {}", pairs.iter()
        .filter(|(p1, p2)| overlaps(p1, p2) || overlaps(p2, p1))
        .count()
    );
}

fn is_within(p1: &(u32, u32), p2: &(u32, u32)) -> bool {
    p1.0 >= p2.0 && p1.1 <= p2.1
}

fn overlaps(p1: &(u32, u32), p2: &(u32, u32)) -> bool {
    p1.0 >= p2.0 && p1.0 <= p2.1 ||
    p1.1 >= p2.0 && p1.1 <= p2.1
}
