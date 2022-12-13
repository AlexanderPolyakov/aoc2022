use std::fs;
use std::str::FromStr;

#[derive(Clone,Copy)]
enum Op {
    Add,
    Mult,
    Pow
}



/*
fn proc_old_worry(item : &Vec<i128>, op : Op, operand : &Vec<i128>, primes : &Vec<i128>) -> i128 {
    let mut v = 0;
    for sum in &item.sums {
        v += deprime(&sum, primes);
    }
    let operand_i32 = deprime(operand, primes);
    match op {
        Op::Add => v + operand_i32,
        Op::Mult => v * operand_i32,
        Op::Pow => v * v,
    }
}
*/

// t(kn + m) = tkn + tm mod n
// (kn + m)(kn + m) = n(kkn + km + km) + mm

/*
fn proc_worry(item : &HashMap<i32, u32>, op : Op,
            operand : &HashMap<i32, u32>, primes : &Vec<i32>) -> HashMap<i32, u32> {
    let mut ret = item.clone();
    match op {
        Op::Add => {
            // TODO: do everything!!!
            let mut lcd : HashMap<i32, u32> = HashMap::new();
            let mut it_rest : i32 = 1;
            let mut op_rest : i32 = 1;
            for (k, v) in item {
                if operand.contains_key(k) {
                    if let Some(op_val) = operand.get(k) {
                        let min_val = cmp::min(v, op_val);
                        lcd.insert(*k, *min_val);
                        it_rest *= k.pow(v - min_val);
                        op_rest *= k.pow(op_val - min_val);
                    }
                }
                else {
                    println!("it_rest {it_rest} {k} {v}");
                    it_rest *= k.pow(*v);
                }
            }
            for (k, v) in operand {
                if !item.contains_key(k) {
                    op_rest *= k.pow(*v);
                }
            }
            let sum = it_rest + op_rest;
            let sum_prime = make_prime(sum, primes);

            let mut addret : HashMap<i32, u32> = lcd.clone();
            for (k, v) in sum_prime {
                addret.entry(k).and_modify(|val| *val += v).or_insert(v);
            }
            return addret;
        },
        Op::Mult => {
            for (k, v) in operand {
                ret.entry(*k).and_modify(|val| *val += v).or_insert(*v);
            }
        },
        Op::Pow => {
            for (_, v) in &mut ret {
                *v *= 2;
            }
        },
    }
    return ret;
}

fn make_primes(count : i128) -> Vec<i32> {
    let mut v : Vec<i32> = Vec::new();
    let mut last_prime = 1;
    for _i in 0..count {
        let mut test_val = last_prime + 1;
        loop {
            let min_bound = (test_val as f32).sqrt() as i32;
            let mut is_prime = true;
            for prime in &v {
                if *prime > min_bound { break; }
                if test_val % prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                v.push(test_val);
                last_prime = test_val;
                break;
            }
            test_val += 1;
        }
    }
    return v;
}

fn make_prime(item : i32, primes : &Vec<i32>) -> HashMap<i32, u32> {
    let mut v : HashMap<i32, u32> = HashMap::new();
    let mut temp_val = item;
    for prime in primes {
        while temp_val % prime == 0 {
            v.entry(*prime).and_modify(|val| *val += 1).or_insert(1);
            temp_val /= prime;
        }
    }
    if temp_val > 1 {
        // go and find a prime again!
        v.insert(temp_val, 1);
    }
    return v;
}
*/

/*
fn deprime(item : &Vec<i128>, primes : &Vec<i128>) -> i128 {
    let mut val = 1;
    for (i, v) in item.iter().enumerate() {
        for _i in 0..*v {
            val *= primes[i];
        }
    }
    return val;
}

fn test_divisible(item : &HashMap<i32, u32>, divisor : &HashMap<i32, u32>) -> bool {
    for (k, v) in divisor {
        if !item.contains_key(k) { return false; }
        if let Some(item_val) = item.get(k) {
            if item_val > v { return false; }
        }
    }
    return true;
}
*/

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

    //let primes = make_primes(50);
    //println!("primes: {:?}", primes);

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
    // ((a*b*c + n) * d*e*f + m) * g*h*i =
    // abcdefghi + ndefghi + mghi
    // ghi(def(abc + n) + m)
    // 2*2*3+3=2*3*3
    for i in 0..10000 {
        println!("round {i}");
        for monkey_id in 0..monkeys.len() {
            //println!("monkey {monkey_id}");
            let mut monkey = &mut monkeys[monkey_id];
            let mut ops : Vec<(Vec<i32>, usize)> = Vec::new();
            for item in &monkey.items {
                let new_item = proc_worry(item, monkey.op, monkey.operand, &divisors);
                let is_divisible = new_item[monkey_id] == 0;//test_divisible(new_item, &monkey.test_div);
                /*
                if i <= 9 {
                    let new_worry = proc_old_worry(item, monkey.op, &monkey.operand, &primes);
                    let mut v = 0;
                    for sum in &new_item.sums {
                        v += deprime(&sum, &primes);
                    }
                    if v != new_worry {
                        println!("Error!!!");
                    }
                }
                */
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
