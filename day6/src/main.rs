use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let chars : Vec<char> = contents.chars().collect();
    let dist = 14;
    for i in dist-1..chars.len() {
        let mut collision = false;
        for j in 0..dist {
            for k in j+1..dist {
                if chars[i-k] == chars[i-j] {
                    collision = true;
                    break;
                }
            }
            if collision { break; }
        }
        if !collision { println!("idx = {i}"); }
    }
}
