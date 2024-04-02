// 解けなかった
struct Solution {}
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();

        vec![1]
    }
}

// AC
// 模範解答
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result = nums;
        for i in 1..result.len() - 1 {
            let (a, b, c) = (result[i - 1], result[i], result[i + 1]);

            if a > b && b > c || a < b && b < c {
                result.swap(i, i + 1);
            }
        }

        result
    }
}

fn main() {
    let case_1 = vec![1, 2, 3, 4, 5];
    // => [1, 2, 4, 5, 3]
    let case_2 = vec![6, 2, 0, 9, 7];
    // => [9, 7, 6, 2, 0]

    println!(
        "case_1: {:?}",
        SolutionAnsCpp::rearrange_array(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionAnsCpp::rearrange_array(case_2.clone())
    );
}
