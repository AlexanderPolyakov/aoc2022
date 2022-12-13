use std::fs;
use std::cmp;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut count = 0;
    let lines : Vec<&str> = contents.lines().collect();
    let height = lines.len();
    let width = lines[0].chars().count();
    /*
    let mut cover_map : Vec<u8> = Vec::new();
    cover_map.resize(width * height, 0);
    */
    let num_base = '0' as i8;
    for (y, line) in lines.iter().enumerate() {
        let mut max_ht : i8 = -1;
        for (x, ch) in line.chars().enumerate() {
            let ht = (ch as i8) - num_base;
            if ht > max_ht {
                max_ht = ht;
                continue;
            }
            let mut covered_right = false;
            for i in x+1..width {
                let rht = (line.chars().nth(i).unwrap() as i8) - num_base;
                if rht >= ht {
                    covered_right = true;
                    break;
                }
            }
            if !covered_right { continue; }
            let mut covered_up = false;
            for i in 0..y {
                let uht = (lines[i].chars().nth(x).unwrap() as i8) - num_base;
                if uht >= ht {
                    covered_up = true;
                    break;
                }
            }
            if !covered_up { continue; }
            let mut covered_down = false;
            for i in y+1..height {
                let dht = (lines[i].chars().nth(x).unwrap() as i8) - num_base;
                if dht >= ht {
                    covered_down = true;
                    break;
                }
            }
            if !covered_down { continue; }
            count += 1;
        }
    }
    let vis_count = width * height - count;
    println!("count = {vis_count}");

    let mut max_score = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let ht = (ch as i8) - num_base;
            let mut left_count = 0;
            let mut right_count = 0;
            let mut up_count = 0;
            let mut down_count = 0;
            for i in 0..x {
                left_count += 1;
                let lht = (line.chars().nth(x-i-1).unwrap() as i8) - num_base;
                if lht >= ht { break; }
            }
            for i in x+1..width {
                let rht = (line.chars().nth(i).unwrap() as i8) - num_base;
                right_count += 1;
                if rht >= ht { break; }
            }
            for i in 0..y {
                let uht = (lines[y-i-1].chars().nth(x).unwrap() as i8) - num_base;
                up_count += 1;
                if uht >= ht { break; }
            }
            for i in y+1..height {
                let dht = (lines[i].chars().nth(x).unwrap() as i8) - num_base;
                down_count += 1;
                if dht >= ht { break; }
            }
            max_score = cmp::max(max_score, left_count * right_count * up_count * down_count);
        }
    }
    println!("max_score = {max_score}");
}
