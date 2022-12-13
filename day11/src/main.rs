use std::fs;
use std::str::FromStr;

#[derive(Clone,Copy)]
enum Op {
    Add,
    Mult,
    Pow
}

struct Monkey {
    items : Vec<Vec<i32>>, // items = [[div0, div1, div2 ..], [div0, div1, div2 ...] ...]
    op : Op,
    operand : i32,
    dest_true : usize,
    dest_false : usize,
    busy : i128,
}

fn proc_worry(item : &Vec<i32>, op : Op, operand : i32, div : &Vec<i32>) -> Vec<i32> {
    let mut ret = item.clone();
    match op {
        Op::Add => {
            for i in 0..ret.len() {
                ret[i] = (ret[i] + operand) % div[i];
            }
        },
        Op::Mult => {
            for i in 0..ret.len() {
                ret[i] = (ret[i] * operand) % div[i];
            }
        },
        Op::Pow => {
            for i in 0..ret.len() {
                ret[i] = (ret[i] * ret[i]) % div[i];
            }
        }
    }
    return ret;
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut monkeys : Vec<Monkey> = Vec::new();
    let mut items : Vec<Vec<i32>> = Vec::new();
    let mut divisors : Vec<i32> = Vec::new();
    for line in contents.lines() {
        if line == "" { continue; }
        let mut tokens = line.split_whitespace();
        let first = tokens.next();
        if Some("Monkey") == first {
            monkeys.push(Monkey{items : Vec::new(),
                                op : Op::Add, operand : 1,
                                dest_true : 0, dest_false : 0, busy: 0});
        }
        else if let Some(last) = monkeys.last_mut() {
            if Some("Starting") == first && Some("items:") == tokens.next() {
                let mut loc_items : Vec<i32> = Vec::new();
                for it in tokens {
                    let item = it.replace(",", "");
                    let item_i32 = i32::from_str(item.as_str()).unwrap();
                    loc_items.push(item_i32);
                }
                items.push(loc_items);
            }
            else if Some("Operation:") == first {
                tokens.next(); // new:
                tokens.next(); // =
                tokens.next(); // old
                let op = tokens.next();
                let operand = tokens.next();
                if Some("+") == op {
                    last.op = Op::Add;
                    let operand_int = i32::from_str(operand.unwrap()).unwrap();
                    last.operand = operand_int;
                }
                else if Some("*") == op && Some("old") == operand { last.op = Op::Pow; }
                else if Some("*") == op {
                    last.op = Op::Mult;
                    let operand_int = i32::from_str(operand.unwrap()).unwrap();
                    last.operand = operand_int;
                }
            }
            else if Some("Test:") == first {
                tokens.next(); // divisible
                tokens.next(); // by
                let divisor = i32::from_str(tokens.next().unwrap()).unwrap();
                divisors.push(divisor);
            }
            else if Some("If") == first {
                let v = tokens.next();
                tokens.next(); tokens.next(); tokens.next();
                let dest = tokens.next();
                let dest_i32 = usize::from_str(dest.unwrap()).unwrap();
                if Some("true:") == v { last.dest_true = dest_i32; }
                else if Some("false:") == v { last.dest_false = dest_i32; }
            }
        }
    }
    // now calculate divisibility and populate monkeys
    for i in 0..monkeys.len() {
        for item in &items[i] {
            let mut div_items : Vec<i32> = Vec::new();
            for j in 0..monkeys.len() {
                div_items.push(item % divisors[j]);
            }
            monkeys[i].items.push(div_items);
        }
    }
    for i in 0..10000 {
        println!("round {i}");
        for monkey_id in 0..monkeys.len() {
            //println!("monkey {monkey_id}");
            let mut monkey = &mut monkeys[monkey_id];
            let mut ops : Vec<(Vec<i32>, usize)> = Vec::new();
            for item in &monkey.items {
                let new_item = proc_worry(item, monkey.op, monkey.operand, &divisors);
                let is_divisible = new_item[monkey_id] == 0;
                let dest = if is_divisible { monkey.dest_true } else { monkey.dest_false };
                ops.push((new_item, dest));
            }
            monkey.busy += monkey.items.len() as i128;
            monkey.items.clear();
            for (it, dest) in ops {
                monkeys[dest].items.push(it);
            }
        }
    }
    for monkey in monkeys {
        println!("busy level = {}", monkey.busy);
    }
}
