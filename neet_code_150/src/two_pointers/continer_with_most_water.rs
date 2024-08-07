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

// 部分的な別解(エッジケースの処理が違う)
struct SolutionLatter {}
impl SolutionLatter {
    // 両側からはさみうち
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut max_area, mut left, mut right) = (0, 0, height.len() - 1);

        while left < right {
            let area = ((right - left) as i32) * height[left].min(height[right]);
            max_area = area.max(max_area);

            if height[left] > height[right] {
                right -= 1;
            } else if height[left] < height[right] {
                left += 1;
                // エッジケース height[left] == height[right]のときの処理は両方動かす処理でもok
            } else {
                right -= 1;
                left += 1;
            }
        }

        max_area
    }

    // ちょっと違う
    // AC
    pub fn max_area_2(height: Vec<i32>) -> i32 {
        let (mut left, mut right) = (0, height.len() - 1);
        let mut result = (right - left) as i32 * height[left].min(height[right]);

        while left < right {
            if height[left] < height[right] {
                left += 1;
            } else if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
                right -= 1;
            }

            result = result.max((right - left) as i32 * height[left].min(height[right]));
        }

        result
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

    println!("case_1: {}", SolutionLatter::max_area(case_1.clone()));
    println!("case_2: {}", SolutionLatter::max_area(case_2.clone()));
    println!("case_3: {}", SolutionLatter::max_area(case_3.clone()));
    println!("case_4: {}", SolutionLatter::max_area(case_4.clone()));
}
