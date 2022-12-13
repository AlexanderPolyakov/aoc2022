use std::fs;
use std::str::FromStr;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut X : i32 = 1;
    let mut cycle = 1;
    let mut sum = 0;
    for line in contents.lines() {
        let mut iter = line.split_whitespace();
        let cmd = iter.next();
        if Some("noop") == cmd {
            if ((cycle - 1) % 40 - X).abs() <= 1 { print!("#"); }
            else { print!("."); }
            if cycle % 40 == 0 { println!(""); }
            cycle += 1;
        }
        else if Some("addx") == cmd {
            let val = i32::from_str(iter.next().unwrap()).unwrap();
            if ((cycle - 1) % 40 - X).abs() <= 1 { print!("#"); }
            else { print!("."); }
            if cycle % 40 == 0 { println!(""); }
            cycle += 1;
            if (cycle + 20) % 40 == 0 {
                sum += X * cycle;
            }
            if ((cycle - 1) % 40 - X).abs() <= 1 { print!("#"); }
            else { print!("."); }
            if cycle % 40 == 0 { println!(""); }
            cycle += 1;
            X += val;
        }
        if (cycle + 20) % 40 == 0 {
            sum += X * cycle;
            
        }
    }
    println!("sum = {sum}");
}
