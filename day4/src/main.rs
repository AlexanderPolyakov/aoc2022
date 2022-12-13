use std::fs;
use std::str::FromStr;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut count = 0u32;
    for line in contents.lines() {
        let arr : Vec<&str> = line.split(',').collect();
        let mut iter = arr.iter();
        let (first, second) = (iter.next().unwrap(), iter.next().unwrap());
        let num_first : Vec<&str> = first.split('-').collect();
        let num_second : Vec<&str> = second.split('-').collect();
        let mut it_first = num_first.iter();
        let mut it_second = num_second.iter();
        let (fb, fe) = (u32::from_str(it_first.next().unwrap()).unwrap(),
                        u32::from_str(it_first.next().unwrap()).unwrap());
        let (sb, se) = (u32::from_str(it_second.next().unwrap()).unwrap(),
                        u32::from_str(it_second.next().unwrap()).unwrap());
        count += if fb >= sb && fb <= se || sb >= fb && sb <= fe ||
                    fe >= sb && fe <= se || se >= fb && se <= fe{ 1 } else {0};
        //println!("{fb}-{fe},{sb}-{se}");
    }
    println!("count {count}");
}
