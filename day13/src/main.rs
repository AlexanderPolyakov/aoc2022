use std::fs;
use core::str::Chars;

fn parse_lists(left : &str, right : &str) -> bool {
    let mut left_depth = 0;
    let mut right_depth = 0;
    let left_ch : Vec<char> = left.chars().collect();
    let right_ch : Vec<char> = right.chars().collect();
    let mut left_head = 0;
    let mut right_head = 0;
    let mut left_i = 0;
    let mut right_i = 0;
    loop {
    }
    return true;
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut lines = contents.lines();
    let mut sum = 0;
    let mut idx = 1;
    loop {
        let left = lines.next().unwrap();
        let right = lines.next().unwrap();

        if parse_lists(left, right) { sum += idx; }

        idx += 1;

        let space = lines.next();
        if space == None { break; }
    }
    println!("sum is {sum}");
}
