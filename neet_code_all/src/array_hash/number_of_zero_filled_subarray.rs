struct Solution {}
impl Solution {
    // TLE: time O(NM) (Mは0ば連続する部分列の長さ)
    // 0が連続する部分列が長くなるとダメ
    pub fn zero_filled_subarray_sq(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut zero_idx = vec![];

        let mut i = 0;
        while i < nums.len() {
            if nums[i] == 0 {
                let mut j = i;
                while j < n && nums[j] == 0 {
                    zero_idx.push((i, j));
                    j += 1;
                }
            }
            i += 1;
        }

        println!("zero_idx:\n {:?}", zero_idx);

        zero_idx.len() as i64
    }

    // AC: time O(N)
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut i = 0;
        let mut result = 0;
        while i < nums.len() {
            if nums[i] == 0 {
                let mut j = i;
                while j < n && nums[j] == 0 {
                    j += 1;
                }
                result += (j - i) * (j + 1 - i) / 2;
                i = j;
            }
            i += 1;
        }

        result as i64
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut count_zero = 0;
        for v in nums {
            if v != 0 {
                result += (1 + count_zero) * count_zero / 2;
                count_zero = 0;
            } else {
                count_zero += 1;
            }
        }

        result += (1 + count_zero) * count_zero / 2;
        result
    }
}

fn main() {
    let case_1 = vec![1, 3, 0, 0, 2, 0, 0, 4];
    // => 6
    let case_2 = vec![0, 0, 0, 2, 0, 0];
    // => 9
    let case_3 = vec![2, 10, 2019];
    // => 0

    println!(
        "case_1: {:?}",
        Solution::zero_filled_subarray(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        Solution::zero_filled_subarray(case_2.clone())
    );
    println!(
        "case_3: {:?}",
        Solution::zero_filled_subarray(case_3.clone())
    );

    println!(
        "case_1: {:?}",
        SolutionAnsCpp::zero_filled_subarray(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionAnsCpp::zero_filled_subarray(case_2.clone())
    );
    println!(
        "case_3: {:?}",
        SolutionAnsCpp::zero_filled_subarray(case_3.clone())
    );
}
