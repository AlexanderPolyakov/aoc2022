use std::env;
use std::fs;
use std::str::FromStr;
use std::cmp;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("In file {}", file_path);
    //println!("Contents:\n{contents}");
    let mut curSum = 0;
    let mut best = vec![0,0,0];
    for line in contents.lines() {
        if line == "" {
            let mut minVal = best[0];
            let mut minIdx = 0;
            for i in 1..best.len() {
                if best[i] < minVal {
                    minVal = best[i];
                    minIdx = i;
                }
            }
            if curSum > minVal {
                best[minIdx] = curSum;
            }
            curSum = 0;
            continue;
        }
        let calorieVal = i32::from_str(line).unwrap();
        println!("{}", calorieVal);
        curSum += calorieVal;
    }
    let mut minVal = best[0];
    let mut minIdx = 0;
    for i in 1..best.len() {
        if best[i] < minVal {
            minVal = best[i];
            minIdx = i;
        }
    }
    if curSum > minVal {
        best[minIdx] = curSum;
    }
    
    let sum = best[0] + best[1] + best[2];
    println!("best sum {sum}");
}

