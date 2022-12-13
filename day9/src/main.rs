use std::fs;
use std::str::FromStr;
use std::cmp;
use std::ops::Sub;
use std::ops::SubAssign;

#[derive(Clone,Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2{x: self.x - other.x, y: self.y - other.y}
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Vec2) {
        *self = Vec2{x: self.x - other.x, y: self.y - other.y};
    }
}

fn match_dir(dir: &str) -> Vec2 {
    match dir {
        "D" => Vec2{x: 0, y: -1},
        "U" => Vec2{x: 0, y: 1},
        "L" => Vec2{x: -1, y: 0},
        "R" => Vec2{x: 1, y: 0},
        _ => Vec2{x: 0, y: 0},
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut lim_min = Vec2{x: 0, y: 0};
    let mut lim_max = Vec2{x: 0, y: 0};
    // figure out initial dimensions
    let mut cur_pos = Vec2{x: 0, y: 0};
    for line in contents.lines() {
        let mut iter = line.split_whitespace();
        let dir = iter.next().unwrap();
        let len = i32::from_str(iter.next().unwrap()).unwrap();
        let dir_vec = match_dir(dir);
        cur_pos.x += dir_vec.x * len;
        cur_pos.y += dir_vec.y * len;
        lim_min.x = cmp::min(lim_min.x, cur_pos.x);
        lim_min.y = cmp::min(lim_min.y, cur_pos.y);
        lim_max.x = cmp::max(lim_max.x, cur_pos.x);
        lim_max.y = cmp::max(lim_max.y, cur_pos.y);
    }
    println!("lims: ({}..{}); ({}..{})", lim_min.x, lim_max.x, lim_min.y, lim_max.y);
    // recenter
    let width = lim_max.x - lim_min.x;
    let height = lim_max.y - lim_min.y;
    let mut head_pos = Vec2{x: -lim_min.x, y: -lim_min.y};
    let mut knot_pos : Vec<Vec2> = Vec::new();
    knot_pos.resize(9, head_pos.clone());
    let mut visited_nodes : Vec<bool> = Default::default();
    visited_nodes.resize((width * height) as usize, false);
    visited_nodes[(head_pos.y * width + head_pos.x) as usize] = true;
    let mut count_visited = 1;
    for line in contents.lines() {
        let mut iter = line.split_whitespace();
        let dir = match_dir(iter.next().unwrap());
        let len = i32::from_str(iter.next().unwrap()).unwrap();
        for _ in 0..len {
            head_pos.x += dir.x;
            head_pos.y += dir.y;
            let mut hpos = head_pos;
            for knot in &mut knot_pos {
                let head_to_knot = hpos - *knot;
                if head_to_knot.x.abs() > 1 || head_to_knot.y.abs() > 1 {
                    // move tail
                    // straight
                    if head_to_knot.x == 0 { knot.y += if head_to_knot.y > 0 { 1 } else { -1 }; }
                    else if head_to_knot.y == 0 { knot.x += if head_to_knot.x > 0 { 1 } else { -1 }; }
                    else {
                        // diagonal
                        knot.x += if head_to_knot.x > 0 { 1 } else { -1 };
                        knot.y += if head_to_knot.y > 0 { 1 } else { -1 };
                    }
                }
                hpos = *knot;
            }
            let tail_pos = knot_pos[8];
            let tail_idx = (tail_pos.y * width + tail_pos.x) as usize;
            if !visited_nodes[tail_idx] {
                count_visited += 1;
                visited_nodes[tail_idx] = true;
            }
        }
    }
    println!("count_visited: {count_visited}");
}
