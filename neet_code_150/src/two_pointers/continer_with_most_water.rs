use std::collections::HashSet;

struct Solution {}
impl Solution {
    // 答えは正しいが O(N^2)でTLE
    pub fn max_area_sq(height: Vec<i32>) -> i32 {
        let area = |i: usize, j: usize| (j - i) as i32 * std::cmp::min(height[i], height[j]);
        let mut area_max = -(1 << 30);

        for i in 0..height.len() {
            for j in (i + 1)..height.len() {
                if area_max < area(i, j) {
                    area_max = area(i, j);
                }
            }
        }
        area_max
    }

    // O(N): 同じ数値が複数あるとダメになる
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut idx_max, mut h_max) = (0, -(1 << 30));
        let (mut idx_max_2, mut h_max_2) = (0, -(1 << 30));

        for i in 0..height.len() {
            if h_max < height[i] {
                h_max = height[i];
                idx_max = i;
            }

            if h_max_2 < height[i] && height[i] < h_max {
                h_max_2 = height[i];
                idx_max_2 = i;
            }
        }
        eprintln!("(idx_max, h_max)= ({},{})", idx_max, h_max);
        eprintln!("(idx_max_2, h_max_2)= ({},{})", idx_max_2, h_max_2);
        i32::abs(idx_max as i32 - idx_max_2 as i32) * h_max_2
    }
}

struct SolutionAns {}
impl SolutionAns {
    // 両側からはさみうち
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut max_area, mut l, mut r) = (0, 0, height.len() - 1);

        while l < r {
            let area = ((r - l) as i32) * height[l].min(height[r]);
            max_area = area.max(max_area);

            if height[l] > height[r] {
                r -= 1;
            } else {
                l += 1;
            }
        }

        max_area
    }
}

fn main() {
    let case_1 = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let case_2 = vec![1, 1];
    let case_3 = vec![1, 1, 1, 1];
    let case_4 = vec![3, 1, 1, 3];

    println!("case_1: {}", Solution::max_area_sq(case_1.clone()));
    println!("case_2: {}", Solution::max_area_sq(case_2.clone()));
    println!("case_3: {}", Solution::max_area_sq(case_3.clone()));
    println!("case_4: {}", Solution::max_area_sq(case_4.clone()));

    /*
    println!("case_1: {}", Solution::max_area(case_1.clone()));
    println!("case_2: {}", Solution::max_area(case_2.clone()));
    println!("case_3: {}", Solution::max_area(case_3.clone()));
    println!("case_4: {}", Solution::max_area(case_4.clone()));
    */

    println!("case_1: {}", SolutionAns::max_area(case_1.clone()));
    println!("case_2: {}", SolutionAns::max_area(case_2.clone()));
    println!("case_3: {}", SolutionAns::max_area(case_3.clone()));
    println!("case_4: {}", SolutionAns::max_area(case_4.clone()));
}
