use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let a_base = 'a' as u32;
    let a_cap_base = 'A' as u32;
    let mut sum = 0u32;
    let mut lines = contents.lines();
    loop {
        let (ln0, ln1, ln2) = (lines.next(), lines.next(), lines.next());
        if ln0 == None { break; }
        let mut found = false;
        for ch in ln0.unwrap().chars() {
            let ch_code = ch as u32;
            for chj in ln1.unwrap().chars() {
                if ch != chj { continue; }
                for chk in ln2.unwrap().chars() {
                    if chk != ch { continue; }
                    if ch_code >= a_base { sum += ch_code - a_base + 1; }
                    else { sum += ch_code - a_cap_base + 27; }
                    found = true;
                    break;
                }
                break;
            }
            if found { break; }
        }
    }
    println!("sum is {sum}");
}
