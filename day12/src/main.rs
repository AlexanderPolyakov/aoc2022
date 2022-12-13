use std::fs;

#[derive(Copy, Clone, PartialEq)]
struct Vec2 {
    x : i8,
    y : i8,
}

/*
fn heuristic(from : Vec2, to : Vec2) -> f32 {
    let diffX = (from.x - to.x) as f32;
    let diffY = (from.y - to.y) as f32;
    return (diffX * diffX + diffY * diffY).sqrt();
}
*/

fn find_path(hmap : &Vec<u8>, width : usize, height : usize, from : Vec2, to : Vec2) -> Vec<Vec2> {
    let heuristic = |lhs : &Vec2, rhs : &Vec2| {
        let diffX = (lhs.x - rhs.x) as f32;
        let diffY = (lhs.y - rhs.y) as f32;
        return (diffX * diffX + diffY * diffY).sqrt();
    };


    let to_coord = |coord : &Vec2| (coord.y as usize) * width + coord.x as usize;

    let mut g : Vec<f32> = Vec::new();
    let mut f : Vec<f32> = Vec::new();
    let mut prev : Vec<Vec2> = Vec::new();

    g.resize(width * height, f32::MAX);
    f.resize(width * height, f32::MAX);
    prev.resize(width * height, Vec2{x: -1, y: -1});

    g[to_coord(&from)] = 0.0;
    f[to_coord(&from)] = heuristic(&from, &to);

    let mut openList : Vec<Vec2> = Vec::new();
    let mut closedList : Vec<Vec2> = Vec::new();
    openList.push(from);

    while openList.len() > 0 {
        let mut best_idx : usize = 0;
        let mut best_val = f32::MAX;
        for (i, vec) in openList.iter().enumerate() {
            if f[to_coord(vec)] > best_val {
                best_idx = i;
                best_val = f[to_coord(vec)];
            }
        }
        let curPos = openList[best_idx];
        if curPos.x == to.x && curPos.y == to.y {
            let reconstruct_path = || {
                let mut curPos = to;
                let mut path : Vec<Vec2> = Vec::new();
                path.push(curPos);
                while prev[to_coord(&curPos)] != (Vec2{x: -1, y: -1}) {
                    curPos = prev[to_coord(&curPos)];
                    path.push(curPos);
                }
                return path;
            };
            return reconstruct_path();
        }
        openList.remove(best_idx);
        if closedList.iter().find(|v| **v == curPos) != None { continue; }

        closedList.push(curPos.clone());
        let curHt = hmap[to_coord(&curPos)];

        let mut check_nei = |p : Vec2| {
            if p.x < 0 || p.x >= width as i8 || p.y < 0 || p.y >= height as i8 { return; }

            let idx = to_coord(&p);
            let moveHt = hmap[idx];
            if moveHt - 1 > curHt { return; } // to high to go here from this direction
            let gscore = g[to_coord(&curPos)] + 1.0;
            if gscore < g[idx] {
                prev[idx] = curPos.clone();
                g[idx] = gscore;
                f[idx] = gscore + heuristic(&p, &to);
            }
            if openList.iter().find(|v| **v == p) == None {
                openList.push(p);
            }
        };
        check_nei(Vec2{x: curPos.x - 1, y: curPos.y + 0});
        check_nei(Vec2{x: curPos.x + 1, y: curPos.y + 0});
        check_nei(Vec2{x: curPos.x + 0, y: curPos.y - 1});
        check_nei(Vec2{x: curPos.x + 0, y: curPos.y + 1});
    }
    return Vec::new(); // empty result, but maybe option/result?
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");
    let height = contents.lines().count();
    let width = contents.lines().next().unwrap().chars().count();
    println!("map is {width}x{height}");

    let mut startPt = Vec2{x: 0, y: 0};
    let mut endPt = Vec2{x: 0, y: 0};

    let mut hmap : Vec<u8> = Vec::new();
    hmap.resize(width * height, 0);
    for (y, line) in contents.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                startPt = Vec2{x: x as i8, y: y as i8};
                hmap[y * width + x] = 'a' as u8;
            }
            else if ch == 'E' {
                endPt = Vec2{x: x as i8, y: y as i8};
                hmap[y * width + x] = 'z' as u8;
            }
            else { hmap[y * width + x] = ch as u8; }
        }
    }

    // a*
    let path = find_path(&hmap, width, height, startPt, endPt);
    let pathLen = path.len() - 1;
    println!("path len {pathLen}");

    let startHt = 'a' as u8;
    let mut shortestPath = usize::MAX;
    for y in 0..height {
        for x in 0..width {
            if hmap[y * width + x] != startHt { continue; }
            let path = find_path(&hmap, width, height, Vec2{x: x as i8, y: y as i8}, endPt);
            if path.len() < shortestPath && path.len() > 0 {
                println!("find shortest: {}", path.len());
                shortestPath = path.len();
            }
        }
    }
    println!("shortestPath = {shortestPath}");
}
