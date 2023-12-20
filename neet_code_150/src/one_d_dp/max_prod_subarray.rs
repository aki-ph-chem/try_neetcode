// 解けなかった
struct Solution {}
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut res = 0;
        let mut res_prev = 0;

        let (mut left, mut right) = (0, n - 1);

        for i in 0..n {
            // even length
            let (mut l, mut r) = (i, i);
            while 0 <= l && r < n {
                l += 1;
                r -= 1;
            }

            // odd length
            (l, r) = (i, i + 1);
            while 0 <= l && r < n {
                l += 1;
                r -= 1;
            }
        }

        res
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    // ?
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut res, mut big, mut small) = (*nums.iter().max().unwrap(), 1, 1);
        for n in nums {
            let tmp = big;
            big = vec![n, big * n, small * n].into_iter().max().unwrap();
            small = vec![n, tmp * n, small * n].into_iter().min().unwrap();
            res = res.max(big);
        }
        res
    }

    // C++の模範解答より
    // AC
    pub fn max_product_2(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let (mut current_max, mut current_min) = (1, 1);

        for n in nums {
            let tmp = n * current_max;
            current_max = std::cmp::max(std::cmp::max(current_max * n, current_min * n), n);
            current_min = std::cmp::min(std::cmp::min(tmp, current_min * n), n);

            res = res.max(current_max);
        }

        res
    }
}

fn main() {
    let case_1 = vec![2, 3, -2, 4];
    // 6
    let case_2 = vec![-2, 0, -1];
    // 0
}
