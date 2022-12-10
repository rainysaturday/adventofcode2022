use std::collections::HashSet;

type Point = (i32, i32);

fn main() {
    let instrs = include_str!("../../../input09")
        .lines()
        .map(|l| l.split_once(" ").expect("Must have space"))
        .map(|(d, v)| (d, v.parse::<i32>().expect("must be a number")))
        .collect::<Vec<(&str, i32)>>();
    
    let mut p1_rope: Vec<Point> = (0..2).map(|_| (0, 0)).collect();
    let mut p1_visited: HashSet<Point> = HashSet::new();
    instrs.iter().for_each(|i| run_instr(i, &mut p1_rope, &mut p1_visited));
    println!("Part1: tail visited {} unique locations", p1_visited.len());

    let mut p2_rope: Vec<Point> = (0..10).map(|_| (0, 0)).collect();
    let mut p2_visited: HashSet<Point> = HashSet::new();
    instrs.iter().for_each(|i| run_instr(i, &mut p2_rope, &mut p2_visited));
    println!("Part2: tail visited {} unique locations", p2_visited.len());
}

fn distance(p1: Point, p2: Point) -> i32 {
    let delta_x = p1.0.max(p2.0) - p1.0.min(p2.0);
    let delta_y = p1.1.max(p2.1) - p1.1.min(p2.1);
    delta_x.max(delta_y)
}

fn draw(rope: &Vec<Point>) {
    for y in -14..20 {
        for x in -14..14 {
            let current = (x, y);

            if let Some((i, _)) = rope.iter().enumerate().find(|(_i, p)| &current == *p) {
                print!("{}", i);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn run_instr(instr: &(&str, i32), rope: &mut Vec<Point>, visited: &mut HashSet<Point>) {
    let delta = match instr {
        ("U", _n) => (0, 1),
        ("D", _n) => (0, -1),
        ("R", _n) => (1, 0),
        ("L", _n) => (-1, 0),
        other => panic!("Unknown instruction {:?}", other),
    };

    for _i in 0..instr.1 {
        // Move head
        rope[0] = (rope[0].0 + delta.0, rope[0].1 + delta.1);

        // Move body
        for i in 1..rope.len() {
            let head_link = rope[i - 1];
            let tail_link = rope[i];
    
            if distance(head_link, tail_link) > 1 {
                // Follow
                if head_link.0 < tail_link.0 {
                    rope[i].0 -= 1;
                }
                if head_link.0 > tail_link.0 {
                    rope[i].0 += 1;
                }
                if head_link.1 > tail_link.1 {
                    rope[i].1 += 1;
                }
                if head_link.1 < tail_link.1 {
                    rope[i].1 -= 1;
                }
            }
        }

        // println!("Iter: {}, {:?}", _i, instr);
        // draw(&rope);

        visited.insert(rope[rope.len() - 1]);
    }
}