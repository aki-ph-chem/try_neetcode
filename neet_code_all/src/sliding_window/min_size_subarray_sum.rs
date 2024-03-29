struct Solution {}
impl Solution {
    // O(NlogN)
    // AC
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;

        let mut perfix_sum = vec![0; n as usize + 1];
        perfix_sum[0] = 0;
        for i in 0..n {
            perfix_sum[i as usize + 1] = nums[i as usize] + perfix_sum[i as usize];
        }
        //println!("prefix_sum: {:?}", perfix_sum);
        let mut result = i32::MAX;
        for i in 0..=n {
            let diff = target + perfix_sum[i as usize];

            let (mut left, mut right) = (0, n as i32);
            while left <= right {
                let mid = left + (right - left) / 2;
                if perfix_sum[mid as usize] == diff {
                    result = result.min((mid - i).abs());
                    break;
                }

                if perfix_sum[mid as usize] < diff {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                    result = result.min((mid - i).abs());
                }
            }
        }

        if result == i32::MAX {
            0
        } else {
            result
        }
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut fp, mut sp) = (0, 1);
        let mut sum = nums[0];
        let mut min = i32::MAX;

        while fp != sp {
            if sum >= target {
                min = min.min(sp - fp);
                sum -= nums[fp as usize];
                fp += 1;
            } else {
                if sp < nums.len() as i32 {
                    sum += nums[sp as usize];
                    sp += 1;
                } else {
                    fp += 1;
                }
            }
        }

        if min == i32::MAX {
            return 0;
        }

        min
    }
}

fn main() {
    let case_1 = (7, vec![2, 3, 1, 2, 4, 3]);
    // => 2
    let case_2 = (4, vec![1, 4, 4]);
    // => 1
    let case_3 = (11, vec![1, 1, 1, 1, 1, 1, 1, 1]);
    // => 0
    let case_4 = (11, vec![1, 2, 3, 4, 5]);
    // => 3

    println!(
        "case_1: {}",
        Solution::min_sub_array_len(case_1.0, case_1.1.clone())
    );
    println!(
        "case_2: {}",
        Solution::min_sub_array_len(case_2.0, case_2.1.clone())
    );
    println!(
        "case_3: {}",
        Solution::min_sub_array_len(case_3.0, case_3.1.clone())
    );
    println!(
        "case_4: {}",
        Solution::min_sub_array_len(case_4.0, case_4.1.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::min_sub_array_len(case_1.0, case_1.1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::min_sub_array_len(case_2.0, case_2.1.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::min_sub_array_len(case_3.0, case_3.1.clone())
    );
    println!(
        "case_4: {}",
        SolutionAnsCpp::min_sub_array_len(case_4.0, case_4.1.clone())
    );
}
