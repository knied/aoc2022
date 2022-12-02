use std::fs;

enum HandSign {
    Rock,
    Paper,
    Scissor,
}

// This looks aweful :D
fn score(elf : &Option<HandSign>, me : &Option<HandSign>) -> Option<i32> {
    match elf {
        Some(HandSign::Rock) => match me {
            Some(HandSign::Rock) => Some(3 + 1), // draw
            Some(HandSign::Paper) => Some(6 + 2), // win
            Some(HandSign::Scissor) => Some(0 + 3), // lose
            None => None,
        },
        Some(HandSign::Paper) => match me {
            Some(HandSign::Rock) => Some(0 + 1), // lose
            Some(HandSign::Paper) => Some(3 + 2), // draw
            Some(HandSign::Scissor) => Some(6 + 3), // win
            None => None,
        },
        Some(HandSign::Scissor) => match me {
            Some(HandSign::Rock) => Some(6 + 1), // win
            Some(HandSign::Paper) => Some(0 + 2), // lose
            Some(HandSign::Scissor) => Some(3 + 3), // draw
            None => None,
        },
        None => None,
    }
}

enum Goal {
    Lose,
    Draw,
    Win,
}

fn response(goal : Goal, elf : &Option<HandSign>) -> Option<HandSign> {
    match elf {
        Some(HandSign::Rock) => match goal {
            Goal::Lose => Some(HandSign::Scissor),
            Goal::Draw => Some(HandSign::Rock),
            Goal::Win => Some(HandSign::Paper),
        },
        Some(HandSign::Paper) => match goal {
            Goal::Lose => Some(HandSign::Rock),
            Goal::Draw => Some(HandSign::Paper),
            Goal::Win => Some(HandSign::Scissor),
        },
        Some(HandSign::Scissor) => match goal {
            Goal::Lose => Some(HandSign::Paper),
            Goal::Draw => Some(HandSign::Scissor),
            Goal::Win => Some(HandSign::Rock),
        },
        None => None,
    }
}

fn round(line : &str) -> (Option<HandSign>, Option<HandSign>){
    let chars : Vec<&str> = line.split_whitespace().collect();
    let elf = match chars[0] {
        "A" => Some(HandSign::Rock), // rock
        "B" => Some(HandSign::Paper), // paper
        "C" => Some(HandSign::Scissor), // scissor
        _ => None,
    };
    let me = match chars[1] {
        "X" => response(Goal::Lose, &elf), // need to lose
        "Y" => response(Goal::Draw, &elf), // need to draw
        "Z" => response(Goal::Win, &elf), // need to win
        _ => None,
    };
    return (elf, me);
}

fn main() {
    let content = fs::read_to_string("input/day2a.txt")
        .expect("Unable to read input file.");

    let res : Option<i32> = content
        .lines()
        .map(|line| round(line))
        .map(|(elf, me)| score(&elf, &me))
        .sum();
    println!("{:?}", res);
}
