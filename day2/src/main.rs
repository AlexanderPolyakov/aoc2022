use std::fs;

#[derive(PartialEq)]
enum Choice {
    Rock = 0,
    Paper = 1,
    Scissors = 2
}

enum RoundResult {
    Lose,
    Draw,
    Win
}

impl From<char> for Choice {
    fn from(ch : char) -> Choice {
        match ch {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            _ => Choice::Scissors
        }
    }
}

impl From<char> for RoundResult {
    fn from(ch : char) -> RoundResult {
        match ch {
            'X' => RoundResult::Lose,
            'Y' => RoundResult::Draw,
            _ => RoundResult::Win
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    //println!("Contents:\n{contents}");
    let mut score = 0u32;
    for line in contents.lines() {
        let elf_choice = Choice::from(line.chars().nth(0).unwrap());
        let choice_idx = elf_choice as u32;
        let mut my_choice = choice_idx;
        let round_result = RoundResult::from(line.chars().nth(2).unwrap());
        match round_result {
            RoundResult::Win => {
                score += 6;
                my_choice = (choice_idx + 1) % 3;
            },
            RoundResult::Draw => {
                score += 3;
            },
            _ => my_choice = (choice_idx + 2) % 3
        }
        score += my_choice + 1;
    }
    println!("Your score {score}");
}
