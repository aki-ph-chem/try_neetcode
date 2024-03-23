// AC
struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() as i32 - 1);

        let mut mid = 0;
        while l <= r {
            mid = l + (r - l) / 2;
            if target == nums[mid as usize] {
                return mid;
            }

            if target < nums[mid as usize] {
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }

        //println!("not found");
        if nums[mid as usize] < target {
            return mid + 1;
        }

        mid
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(i) => i as i32,
            Err(i) => i as i32,
        }
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        left
    }
}

fn main() {
    let case_1 = (vec![1, 3, 5, 6], 5);
    // => 2
    let case_2 = (vec![1, 3, 5, 6], 2);
    // => 1
    let case_3 = (vec![1, 3, 5, 6], 7);
    // => 4
    let case_4 = (vec![1, 3, 5, 6], 0);
    // => 0

    println!(
        "case_1: {}",
        Solution::search_insert(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        Solution::search_insert(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        Solution::search_insert(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        Solution::search_insert(case_4.0.clone(), case_4.1)
    );

    println!(
        "case_1: {}",
        SolutionAns::search_insert(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAns::search_insert(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAns::search_insert(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        SolutionAns::search_insert(case_4.0.clone(), case_4.1)
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::search_insert(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::search_insert(case_2.0.clone(), case_2.1)
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::search_insert(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_4: {}",
        SolutionAnsCpp::search_insert(case_4.0.clone(), case_4.1)
    );
}
