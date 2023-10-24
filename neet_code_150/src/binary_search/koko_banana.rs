// 出来なかった😭
struct Solution {}
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut pl = piles;
        pl.sort();
        let mut left = pl.iter().next().unwrap();
        let mut right = pl.iter().rev().next().unwrap();
        /*
        println!("left: {}", left);
        println!("right: {}", right);
        */
        while left <= right {
            let mid = (left + right) / 2;
        }

        3
    }
}

use std::cmp::Ordering::*;
struct SolutionAns {}
impl SolutionAns {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let max_piles = *piles.iter().max().unwrap() as usize;
        let (mut l, mut r) = (1, max_piles);
        let mut k = max_piles;

        while l <= r {
            let m = l + (r - l) / 2;
            let hrs: usize = piles
                .iter()
                .map(|&num_bananas| ((num_bananas - 1) as usize / m) + 1)
                .sum();

            match hrs.cmp(&(h as usize)) {
                Less | Equal => {
                    k = k.min(m);
                    r = m - 1;
                }
                Greater => l = m + 1,
            }
        }

        k as i32
    }
}

fn main() {
    let case_1 = (vec![3, 6, 7, 11], 8);
    let case_2 = (vec![30, 11, 23, 4, 20], 5);
    let case_3 = (vec![30, 11, 23, 4, 20], 6);

    println!("case_1: {}", SolutionAns::min_eating_speed(case_1.0.clone(), case_1.1));
    println!("case_2: {}", SolutionAns::min_eating_speed(case_2.0.clone(), case_2.1));
    println!("case_3: {}", SolutionAns::min_eating_speed(case_3.0.clone(), case_3.1));
}
