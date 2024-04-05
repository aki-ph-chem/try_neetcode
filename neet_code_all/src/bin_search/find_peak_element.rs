// 解けなかった
struct Solution {}
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let (mut x_0, mut x_3) = (0 as i32, nums.len() as i32 - 1);

        while x_3 - x_0 > 0 {
            let diff = (x_3 - x_0).abs() / 3;
            let (x_1, x_2) = (x_0 + diff, x_3 - diff);

            if nums[x_1 as usize] >= nums[x_2 as usize] {
                x_3 = x_2;
            } else {
                x_0 = x_1;
            }

            println!("nums_x_1: {}", nums[x_1 as usize]);
            println!("nums_x_2: {}", nums[x_2 as usize]);
            println!("x_0, x_1, x_2, x_3: {x_0}, {x_1}, {x_2}, {x_3}");
        }

        nums[x_3 as usize]
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        if n == 1 {
            return 0;
        }

        let (mut left, mut right) = (0, n - 1);
        while left <= right {
            let mid = left + (right - left) / 2;

            if mid > 0 && nums[mid as usize] < nums[mid as usize - 1] {
                right = mid - 1;
            } else if mid < n - 1 && nums[mid as usize] < nums[mid as usize + 1] {
                left = mid + 1;
            } else {
                return mid;
            }
        }

        -1
    }

    // AC
    pub fn find_peak_element_2(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        if n == 1 {
            return 0;
        }

        let (mut left, mut right) = (0, n - 1);
        while left <= right {
            let mid = left + (right - left) / 2;

            // returnでスコープを抜けるケースを陽に書くとこうなる
            // ちょっと泥臭い
            if (mid == 0 && nums[mid as usize] > nums[mid as usize + 1])
                || (mid == n - 1 && nums[mid as usize - 1] < nums[mid as usize])
                || (mid > 0
                    && mid < n - 1
                    && nums[mid as usize - 1] < nums[mid as usize]
                    && nums[mid as usize] > nums[mid as usize + 1])
            {
                return mid;
            }

            if mid > 0 && nums[mid as usize - 1] > nums[mid as usize] {
                right = mid - 1;
            } else if mid < n - 1 && nums[mid as usize] < nums[mid as usize + 1] {
                left = mid + 1;
            }
        }

        -1
    }
}

// 縛り: time O(logN)
fn main() {
    let case_1 = vec![1, 2, 3, 1];
    // => 2
    let case_2 = vec![1, 2, 1, 3, 5, 6, 4];
    // => 5 (or 1)
    let case_3 = vec![2, 1];
    // => 0

    /*
    println!("case_1: {}", Solution::find_peak_element(case_1.clone()));
    println!("case_2: {}", Solution::find_peak_element(case_2.clone()));
    */

    println!(
        "case_1: {}",
        SolutionAnsCpp::find_peak_element(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::find_peak_element(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::find_peak_element(case_3.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::find_peak_element_2(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::find_peak_element_2(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::find_peak_element_2(case_3.clone())
    );
}
