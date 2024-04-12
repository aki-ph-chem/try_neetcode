struct Solution {}
impl Solution {
    // AC
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);
        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid as usize] == target {
                //println!("mid: {mid}");
                let (mut l_mid, mut r_mid) = (mid, mid);
                while l_mid - 1 >= 0 && nums[l_mid as usize - 1] == target {
                    l_mid -= 1;
                }
                while r_mid + 1 < nums.len() as i32 && nums[r_mid as usize + 1] == target {
                    r_mid += 1;
                }

                return vec![l_mid, r_mid];
            }

            if nums[mid as usize] < target {
                left = mid + 1;
            } else if nums[mid as usize] > target {
                right = mid - 1;
            }
        }

        vec![-1, -1]
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        let (mut left, mut right) = (0, nums.len() as i32 - 1);
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] >= target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        if nums[left as usize] != target {
            return vec![-1, -1];
        }

        let left_result = left;
        left = 0;
        right = nums.len() as i32 - 1;

        while left < right {
            let mid = left + (right - left + 1) / 2;
            if nums[mid as usize] <= target {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        vec![left_result, right]
    }
}

fn main() {
    let case_1 = (vec![5, 7, 7, 8, 8, 10], 8);
    // => [3, 4]
    let case_2 = (vec![5, 7, 7, 8, 8, 10], 6);
    // => [-1, -1]
    let case_3 = (Vec::<i32>::new(), 0);
    // => [-1, -1]

    println!(
        "case_1: {:?}",
        Solution::search_range(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        Solution::search_range(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        Solution::search_range(case_3.0.clone(), case_3.1)
    );

    println!(
        "case_1: {:?}",
        SolutionAnsCpp::search_range(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        SolutionAnsCpp::search_range(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {:?}",
        SolutionAnsCpp::search_range(case_3.0.clone(), case_3.1)
    );
}
