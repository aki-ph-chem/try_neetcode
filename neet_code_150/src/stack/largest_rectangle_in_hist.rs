// 要するに(部分列の長さ) x (その部分列に含まれる最小値)　の最大値を求める問題

// 解けなかった
struct Solution {}
impl Solution {
    pub fn largest_rectangle_area(height: Vec<i32>) -> i32 {
        let n = height.len();
        //let mut stk: Vec<(usize, i32)> = vec![];
        let mut result = i32::MIN;

        let (mut l, mut r) = (0, 0);
        while r < n {
            if height[l] > height[r] {
                result = result.max((r - l + 1) as i32 * height[r]);
            } else {
                result = result.max((r - l + 1) as i32 * height[l]);
                l = r;
            }

            r += 1;
        }

        println!("{},{}", l, r);

        result
    }
}

// C++の模範解答より
// AC
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn largest_rectangle_area(height: Vec<i32>) -> i32 {
        let mut stk: Vec<(usize, i32)> = vec![];
        let mut result = 0_i32;
        let n = height.len();

        for i in 0..n {
            let mut start = i;

            while !stk.is_empty() && stk.iter().rev().next().unwrap().1 > height[i] {
                let (idx, h) = stk.pop().unwrap();
                let width = i - idx;

                result = result.max(h * width as i32);
                start = idx;
            }

            stk.push((start, height[i]));
        }

        while !stk.is_empty() {
            let (idx, h) = stk.pop().unwrap();
            let width = height.len() - idx;

            result = result.max(h * width as i32);
        }

        result
    }
}

fn main() {
    let case_1 = vec![2, 1, 5, 6, 2, 3];
    // => 10
    let case_2 = vec![2, 4];
    // => 4

    println!(
        "case_1: {}",
        Solution::largest_rectangle_area(case_1.clone())
    );
    println!(
        "case_2: {}",
        Solution::largest_rectangle_area(case_2.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::largest_rectangle_area(case_1.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::largest_rectangle_area(case_2.clone())
    );
}
