#[derive(PartialEq, Clone, Copy)]
enum Item {
    Rock,
    Paper,
    Scissor,
}

impl Item {
    fn value(&self) -> u32 {
        match self {
            Item::Rock => 1,
            Item::Paper => 2,
            Item::Scissor => 3,
        }
    }

    fn from(item: &str) -> Item {
        match item {
            "X" | "A" => Item::Rock,
            "Y" | "B" => Item::Paper,
            "Z" | "C" => Item::Scissor,
            other => panic!("Invalid item {}", other)
        }
    }
}


fn get_losing_answer(against: &Item) -> Item {
    match against {
        Item::Rock => Item::Scissor,
        Item::Paper => Item::Rock,
        Item::Scissor => Item::Paper,
    }
}

fn get_winning_answer(against: &Item) -> Item {
    match against {
        Item::Rock => Item::Paper,
        Item::Paper => Item::Scissor,
        Item::Scissor => Item::Rock,
    }
}

fn score(round: &(Item, (&str, Item)), part2: bool) -> u32 {
    let my_play = if !part2 { round.1.1 } else {
        match round.1.0 {
            "X" =>  get_losing_answer(&round.0),    // Should lose,
            "Y" =>  round.0,                        // Should draw,
            "Z" =>  get_winning_answer(&round.0),   // Should win,
            _ => panic!("Invalid suggestion for end round")
        }
    };

    if my_play == get_winning_answer(&round.0) {
        return my_play.value() + 6;   // Win
    } else if my_play == round.0 {
        return my_play.value() + 3;   // Draw
    } else {
        return my_play.value();       // Lose
    }
}

fn main() {
    let rounds = include_str!("../../../input02").lines().map(|l| {
        let items: Vec<&str> = l.split(" ").collect();
        (Item::from(items[0]), (items[1], Item::from(items[1])))
    }).collect::<Vec<(Item, (&str, Item))>>();

    println!("Part1: Score according to guide {}", rounds.iter().map(|r| score(r, false)).sum::<u32>());
    println!("Part2: Score according to guide {}", rounds.iter().map(|r| score(r, true)).sum::<u32>());

}
