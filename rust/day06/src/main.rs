use std::collections::HashSet;

fn main() {
    let msg = include_str!("../../../input06")
        .trim()
        .chars()
        .collect::<Vec<char>>();

    println!("Part1: {:?}", msg.iter().enumerate().find(|(i, _)| start_of_packet(&msg, *i, 4)).expect("should have start packet").0 + 1);
    println!("Part2: {:?}", msg.iter().enumerate().find(|(i, _)| start_of_packet(&msg, *i, 14)).expect("should have start packet").0 + 1);
}

fn start_of_packet(msg: &Vec<char>, pos: usize, length: usize) -> bool {
    let mut set = HashSet::new();
    let start = if pos < length { 0 } else { pos - length + 1 };
    for i in start..=pos {
        set.insert(msg[i]);
    }

    set.len() == length
}