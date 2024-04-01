use std::collections::HashMap;
use std::collections::HashSet;

struct Solution {}
impl Solution {
    // TLE
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut rct_list = rectangles;

        for rct in rct_list.iter_mut() {
            let gcd_rct = Self::gcd(rct[0], rct[1]);
            rct[0] /= gcd_rct;
            rct[1] /= gcd_rct;
        }
        let mut result = 0;

        for i in 0..rct_list.len() {
            let mut set = HashSet::new();
            set.insert(rct_list[i].clone());
            for j in i + 1..rct_list.len() {
                if set.contains(&rct_list[j]) {
                    result += 1;
                }
            }
            set.remove(&rct_list[i]);
        }

        result
    }

    // AC
    pub fn interchangeable_rectangles_2(rectangles: Vec<Vec<i32>>) -> i64 {
        let rct_list = rectangles;

        let mut map: HashMap<Vec<i32>, Vec<usize>> = HashMap::new();
        for (i, rct) in rct_list.iter().enumerate() {
            let gcd_rct = Self::gcd(rct[0], rct[1]);
            let rct_reduced = vec![rct[0] / gcd_rct, rct[1] / gcd_rct];

            if let Some(map_key) = map.get_mut(&rct_reduced) {
                map_key.push(i);
            } else {
                map.insert(rct_reduced, vec![i]);
            }
        }
        //println!("map:\n {:?}", map);

        let mut result = 0;
        for (_v, idx_list) in map {
            let n = idx_list.len() as i64;
            result += n * (n - 1) / 2;
        }

        result
    }

    // AC(2をもう少しシンプルにした)
    pub fn interchangeable_rectangles_3(rectangles: Vec<Vec<i32>>) -> i64 {
        let rct_list = rectangles;

        let mut map: HashMap<(i32, i32), usize> = HashMap::new();
        for rct in rct_list.iter() {
            let gcd_rct = Self::gcd(rct[0], rct[1]);
            let rct_reduced = (rct[0] / gcd_rct, rct[1] / gcd_rct);

            *map.entry(rct_reduced).or_default() += 1;
        }
        //println!("map:\n {:?}", map);

        let mut result = 0;
        for (_v, num) in map {
            result += num * (num - 1) / 2;
        }

        result as i64
    }

    fn gcd(a: i32, b: i32) -> i32 {
        match b {
            0 => a,
            _ => Self::gcd(b, a % b),
        }
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    // AC
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        rectangles
            .into_iter()
            .map(|v| {
                let gcd = SolutionAns::compute_gcd(v[0], v[1]);
                (v[0] / gcd, v[1] / gcd)
            })
            .fold(HashMap::new(), |mut dict, t| {
                dict.entry(t).and_modify(|cnt| *cnt += 1).or_insert(1);
                dict
            })
            .into_iter()
            .filter(|(_, v)| *v > 1)
            .map(|(_, v)| v)
            .fold(0, |mut cnt, v| {
                cnt += v * (v - 1) / 2;
                cnt
            })
    }

    fn compute_gcd(mut a: i32, mut b: i32) -> i32 {
        while a > 0 && b > 0 {
            if a > b {
                a %= b;
            } else {
                b %= a;
            }
        }

        if a == 0 {
            b
        } else {
            a
        }
    }
}

fn main() {
    let case_1 = vec![vec![4, 8], vec![3, 6], vec![10, 20], vec![15, 30]];
    // => 6
    let case_2 = vec![vec![4, 5], vec![7, 8]];
    // => 0

    println!(
        "case_1: {}",
        Solution::interchangeable_rectangles(case_1.clone())
    );
    println!(
        "case_2: {}",
        Solution::interchangeable_rectangles(case_2.clone())
    );

    println!(
        "case_1: {}",
        Solution::interchangeable_rectangles_2(case_1.clone())
    );
    println!(
        "case_2: {}",
        Solution::interchangeable_rectangles_2(case_2.clone())
    );

    println!(
        "case_1: {}",
        Solution::interchangeable_rectangles_3(case_1.clone())
    );
    println!(
        "case_2: {}",
        Solution::interchangeable_rectangles_3(case_2.clone())
    );

    println!(
        "case_1: {}",
        SolutionAns::interchangeable_rectangles(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAns::interchangeable_rectangles(case_2.clone())
    );
}
