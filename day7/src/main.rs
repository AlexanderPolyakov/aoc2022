use std::fs;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let mut curdir : Vec<&str> = Vec::new();
    let mut dirsz : HashMap<String, u32> = HashMap::new();
    for line in contents.lines() {
        let mut iter = line.split_whitespace();
        let first = iter.next();
        if Some("$") == first { // command
            let cmd = iter.next();
            if Some("cd") == cmd { // changing directory
                let dir = iter.next();
                if Some("..") == dir { curdir.pop(); }
                else if Some("/") == dir { curdir.push(""); }
                else {
                    let d = dir.unwrap();
                    curdir.push(d);
                }
                let mut dd = String::new();
                for d in &curdir {
                    dd.push_str(d);
                    dd.push_str("/");
                }
                if !dirsz.contains_key(&dd) { dirsz.insert(dd, 0); }
            }
            else if Some("ls") == cmd { // listing
                // nop!
            }
        }
        else if Some("dir") == first { // dir
            // nop again it seems, as we don't use this info
        }
        else { // file
            let sz = u32::from_str(first.unwrap()).unwrap();
            let mut path = String::new();
            for d in &curdir {
                path.push_str(d);
                path.push_str("/");
                dirsz.entry(path.clone()).and_modify(|val| *val += sz).or_insert(sz);
            }
        }
    }
    let mut count = 0;
    for (key, val) in dirsz.iter() {
        println!("{key} = {val}");
        count += if val <= &100000 { val } else { &0 };
    }
    let sz = 70000000;
    let need = 30000000;
    let cursz = dirsz.get("/").unwrap();
    let thres = need - (sz - cursz);
    let mut minimal = sz;
    for (key, val) in dirsz.iter() {
        if val < &thres { continue; }
        if val < &minimal { minimal = *val; }
    }
    println!("thres {thres}");
    println!("count = {count}");
    println!("minimal {minimal}");
}
