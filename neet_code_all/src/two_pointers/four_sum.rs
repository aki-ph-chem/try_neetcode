// 解けなかった
struct Solution {}
impl Solution {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let n = nums.len();
        if n < 4 {
            return result;
        }

        nums.sort();
        for i in 0..n - 3 {
            for j in i + 1..n - 2 {
                if nums[i] + nums[j] > target {
                    break;
                }

                if (i >= 1 && nums[i - 1] == nums[i]) || (j >= 1 && nums[j - 1] == nums[j]) {
                    continue;
                }

                let (mut l, mut m) = (j + 1, n - 1);
                while l < m {
                    let sum = nums[i] + nums[j] + nums[l] + nums[m];

                    if sum < target {
                        l += 1;
                    } else if sum > target {
                        m -= 1;
                    } else {
                        result.push(vec![nums[i], nums[j], nums[l], nums[m]]);
                        break;
                    }
                }
            }
        }

        result
    }

    pub fn four_sum_2(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        if nums.len() < 4 {
            return result;
        }
        nums.sort();

        let (mut l_1, mut r_1) = (0, nums.len() - 1);
        while l_1 < r_1 {
            let (mut l_2, mut r_2) = (l_1 + 1, r_1 - 1);
            while l_2 < r_2 {
                let sum = nums[l_1] + nums[r_1] + nums[l_2] + nums[r_2];
            }
        }

        result
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let target = target as i64;
        nums.sort();

        let mut result = vec![];
        let n = nums.len() as i32;

        for i in 0..n {
            if i > 0 && nums[i as usize] == nums[i as usize - 1] {
                continue;
            }
            for j in (i + 1)..n {
                if j > i + 1 && nums[j as usize] == nums[j as usize - 1] {
                    continue;
                }

                let (mut l, mut r) = (j + 1, n - 1);
                while l < r {
                    let sum = nums[i as usize] as i64
                        + nums[j as usize] as i64
                        + nums[l as usize] as i64
                        + nums[r as usize] as i64;
                    if sum == target {
                        result.push(vec![
                            nums[i as usize],
                            nums[j as usize],
                            nums[l as usize],
                            nums[r as usize],
                        ]);
                        l += 1;

                        while l < r && nums[l as usize] == nums[l as usize - 1] {
                            l += 1;
                        }
                    } else if sum > target {
                        r -= 1;
                    } else {
                        l += 1;
                    }
                }
            }
        }

        result
    }
}

fn main() {
    let case_1 = (vec![1, 0, -1, 0, -2, 2], 0);
    // =>  [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
    let case_2 = (vec![2, 2, 2, 2, 2], 8);
    // => [[2,2,2,2]]
    let case_3 = (
        vec![1000000000, 1000000000, 1000000000, 1000000000],
        -294967296,
    );
    // => []

    /*
    println!(
        "case_1: {:?}",
        Solution::four_sum(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        Solution::four_sum(case_2.0.clone(), case_2.1)
    );
    */

    println!(
        "case_1: {:?}",
        SolutionAnsCpp::four_sum(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        SolutionAnsCpp::four_sum(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        SolutionAnsCpp::four_sum(case_3.0.clone(), case_3.1)
    );
}
