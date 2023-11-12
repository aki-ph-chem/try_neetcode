use std::collections::HashMap;
// 解けなかった
#[derive(Debug)]
struct DetectSquares {
    point: Vec<(i32, i32)>,
}

impl DetectSquares {
    fn new() -> Self {
        DetectSquares { point: vec![] }
    }

    fn add(&mut self, point: Vec<i32>) {
        self.point.push((point[0], point[1]));
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let p_x = point[0];
        let p_y = point[1];
        let mut number = 0;
        for p in &self.point {
            let mut len_sq = 0;
            if p.1 == p_y {
                len_sq = p_y - p.1;
            }
        }

        number
    }
}

// 模範解答
struct DetectSquaresAns {
    point: Vec<(i32, i32)>,
    counts: HashMap<(i32, i32), i32>,
}
impl DetectSquaresAns {
    fn new() -> Self {
        Self {
            point: vec![],
            counts: HashMap::new(),
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        let p = (point[0], point[1]);
        self.point.push(p);
        *self.counts.entry(p).or_default() += 1;
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let mut res = 0;
        let (px, py) = (point[0], point[1]);
        for (x, y) in self.point.iter() {
            if (py - y).abs() != (px - x).abs() || *x == px || *y == py {
                continue;
            }

            res +=
                self.counts.get(&(*x, py)).unwrap_or(&0) * self.counts.get(&(px, *y)).unwrap_or(&0);
        }

        res
    }
}

// pointはなしでcountsだけでもできるのでは?
// 今の所ACしない
struct DetectSquareB {
    counts: HashMap<(i32, i32), i32>,
}

impl DetectSquareB {
    fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    fn add(&mut self, point: Vec<i32>) {
        let p = (point[0], point[1]);
        *self.counts.entry(p).or_default() += 1;
    }

    fn count(&self, point: Vec<i32>) -> i32 {
        let mut res = 0;
        let (px, py) = (point[0], point[1]);
        for ((x, y), _c) in self.counts.iter() {
            if (py - y).abs() != (px - x).abs() || *x == px || *y == py {
                continue;
            }

            res +=
                self.counts.get(&(*x, py)).unwrap_or(&0) * self.counts.get(&(px, *y)).unwrap_or(&0);
        }

        res
    }
}

fn main() {
    let mut p_1 = DetectSquares::new();
    p_1.add(vec![3, 10]);
    p_1.add(vec![11, 2]);
    p_1.add(vec![3, 2]);

    p_1.count(vec![11, 10]);
    p_1.count(vec![14, 8]);

    p_1.add(vec![11, 2]);

    p_1.count(vec![11, 10]);

    let mut p_2 = DetectSquaresAns::new();
    p_2.add(vec![3, 10]);
    p_2.add(vec![11, 2]);
    p_2.add(vec![3, 2]);
    println!("{}", p_2.count(vec![11, 10]));
    println!("{}", p_2.count(vec![14, 8]));
    p_2.add(vec![11, 2]);
    println!("{}", p_2.count(vec![11, 10]));

    let mut p_3 = DetectSquareB::new();
    p_3.add(vec![3, 10]);
    p_3.add(vec![11, 2]);
    p_3.add(vec![3, 2]);
    println!("{}", p_3.count(vec![11, 10]));
    println!("{}", p_3.count(vec![14, 8]));
    p_3.add(vec![11, 2]);
    println!("{}", p_3.count(vec![11, 10]));
}
