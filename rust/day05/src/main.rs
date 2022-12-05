type Stacks = Vec<Vec<char>>;

fn main() {
    let lines = include_str!("../../../input05")
        .lines()
        .collect::<Vec<&str>>();

    let mut stacks: Stacks = Vec::new();

    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    // Parse instructions
    let instrs = lines.iter()
        .filter(|l| l.contains("move"))
        .map(|l| l.split(" ")
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .filter_map(|p| p.parse::<usize>().map_or(None, |v| Some(v)))
            .collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    // Parse stacks
    lines.iter().filter(|l| l.contains("[")).for_each(|l| {
        l.chars().enumerate().for_each(|(i, c)| match c {
                'A'..='Z' => {
                    let column = (i - 1) / 4;
                    if column >= 9 {
                        panic!("Too many parsed columns")
                    }
                    stacks[column].push(c);
                }
                _ => { /* Ignore */ }
            })
    });

    // Done parsing, lets flip each column so that the top item is at the highest index
    for i in 0..stacks.len() {
        stacks[i] = stacks[i].iter().rev().cloned().collect();
    }

    let mut p1_stacks = stacks.clone();
    let mut p2_stacks = stacks.clone();

    instrs.iter().for_each(|i| run_instr(i, &mut p1_stacks));
    println!("Part1: head of each stack prints: {}", p1_stacks.iter()
        .map(|s| s.last().unwrap_or(&' '))
        .collect::<String>());

    instrs.iter().for_each(|i| run_instr2(i, &mut p2_stacks));
    println!("Part2: head of each stack prints: {}", p2_stacks.iter()
        .map(|s| s.last().unwrap_or(&' '))
        .collect::<String>());
}

fn run_instr(instr: &Vec<usize>, stacks: &mut Stacks) {
    let count = instr[0];
    let from = instr[1] - 1;
    let to = instr[2] - 1;

    for _ in 0..count {
        let val = stacks[from].pop().expect(&format!("Stack {}, no longer had any items", to));
        stacks[to].push(val);
    }
}

fn run_instr2(instr: &Vec<usize>, stacks: &mut Stacks) {
    let count = instr[0];
    let from = instr[1] - 1;
    let to = instr[2] - 1;

    let from_len = stacks[from].len();
    let mut to_move = stacks[from][(from_len - count)..from_len].iter().cloned().collect::<Vec<char>>();
    stacks[to].append(&mut to_move);
    stacks[from].resize(from_len - count, 'å');
}