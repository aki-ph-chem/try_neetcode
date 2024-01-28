// 解けなかった
struct Solution {}
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        // 左側でsort
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        println!("intervals: {:?}", intervals);
        let n = intervals.len();
        let mut result = 0;

        let mut interval_prev = intervals[0].clone();
        for i in 1..n {
            if intervals[i][0] < interval_prev[1] {
                result += 1;
            } else {
                interval_prev = intervals[i].clone();
            }
        }

        result
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    // AC
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();
        if n == 1 {
            return 0;
        }

        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut result = 0;

        //i : 0 ~ n - 2
        // i 番目 とi + 1 番目を比較
        let mut i = 0;
        while i < n - 1 {
            if intervals[i][1] > intervals[i + 1][0] {
                if intervals[i][1] < intervals[i + 1][1] {
                    intervals[i + 1] = intervals[i].clone();
                }
                result += 1;
            }
            i += 1;
        }

        result
    }

    // AC
    pub fn erase_overlap_intervals_2(mut intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();
        if n == 1 {
            return 0;
        }

        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut result = 0;

        // i: 1 ~ n - 1
        // i - 1 番目とi番目を比較
        let mut i = 1;
        while i < n {
            if intervals[i - 1][1] > intervals[i][0] {
                if intervals[i - 1][1] < intervals[i][1] {
                    intervals[i] = intervals[i - 1].clone();
                }
                result += 1;
            }
            i += 1;
        }

        result
    }

    pub fn erase_overlap_intervals_3(mut intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();
        if n == 1 {
            return 0;
        }

        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut result = 0;

        for i in 1..n {
            if intervals[i - 1][1] > intervals[i][0] {
                if intervals[i - 1][1] < intervals[i][1] {
                    intervals[i] = intervals[i - 1].clone();
                }
                result += 1;
            }
        }

        result
    }
}

fn main() {
    let case_1 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
    // => 1
    let case_1_2 = vec![vec![1, 2], vec![1, 3], vec![2, 3], vec![3, 4]];
    // => 1
    let case_2 = vec![vec![1, 2], vec![1, 2], vec![1, 2]];
    // => 2
    let case_3 = vec![vec![1, 2], vec![2, 3]];
    // => 0
    let case_4 = vec![vec![1, 100], vec![11, 22], vec![1, 11], vec![2, 12]];
    // => 2

    /*
    println!(
        "case_1: {}",
        Solution::erase_overlap_intervals(case_1.clone())
    );
    println!(
        "case_1_2: {}",
        Solution::erase_overlap_intervals(case_1_2.clone())
    );
    println!(
        "case_2: {}",
        Solution::erase_overlap_intervals(case_2.clone())
    );
    println!(
        "case_3: {}",
        Solution::erase_overlap_intervals(case_3.clone())
    );
    println!(
        "case_4: {}",
        Solution::erase_overlap_intervals(case_4.clone())
    );
    */

    println!(
        "case_1: {}",
        SolutionAnsCpp::erase_overlap_intervals(case_1.clone())
    );
    println!(
        "case_1_2: {}",
        SolutionAnsCpp::erase_overlap_intervals(case_1_2.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::erase_overlap_intervals(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::erase_overlap_intervals(case_3.clone())
    );
    println!(
        "case_4: {}",
        SolutionAnsCpp::erase_overlap_intervals(case_4.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::erase_overlap_intervals_2(case_1.clone())
    );
    println!(
        "case_1_2: {}",
        SolutionAnsCpp::erase_overlap_intervals_2(case_1_2.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::erase_overlap_intervals_2(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::erase_overlap_intervals_2(case_3.clone())
    );
    println!(
        "case_4: {}",
        SolutionAnsCpp::erase_overlap_intervals_2(case_4.clone())
    );

    println!(
        "case_1: {}",
        SolutionAnsCpp::erase_overlap_intervals_3(case_1.clone())
    );
    println!(
        "case_1_2: {}",
        SolutionAnsCpp::erase_overlap_intervals_3(case_1_2.clone())
    );
    println!(
        "case_2: {}",
        SolutionAnsCpp::erase_overlap_intervals_3(case_2.clone())
    );
    println!(
        "case_3: {}",
        SolutionAnsCpp::erase_overlap_intervals_3(case_3.clone())
    );
    println!(
        "case_4: {}",
        SolutionAnsCpp::erase_overlap_intervals_3(case_4.clone())
    );
}
