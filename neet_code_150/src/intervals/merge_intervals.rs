// 解けなかった
struct Solution {}
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 1 {
            return intervals;
        }

        let mut result = vec![];
        let n = intervals.len();

        let mut left_next = 0;
        let mut right_next = 0;
        for i in 0..(n - 1) {
            let left = intervals[i][0];
            let right = intervals[i][1];

            left_next = intervals[i + 1][0];
            right_next = intervals[i + 1][1];

            if right < left_next {
                result.push(vec![left, right]);
            } else {
                result.push(vec![left, right.max(right_next)]);
            }
        }
        result.push(vec![left_next, right_next]);

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let acc = vec![intervals.first().unwrap().clone()];

        intervals.into_iter().skip(1).fold(acc, |mut acc, e| {
            let last = acc.last().unwrap();
            if e[0] <= last[1] {
                acc.last_mut().unwrap()[1] = last[1].max(e[1]);
            } else {
                acc.push(e)
            }
            acc
        })
    }
}

//AC
//C++の模範解答
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = intervals.len();
        if n == 1 {
            return intervals;
        }

        // 右側でsort
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut result = vec![];

        let mut i = 0;
        while i < n - 1 {
            // オーバーラップしている場合
            if intervals[i][1] >= intervals[i + 1][0] {
                intervals[i + 1][0] = intervals[i][0];
                intervals[i + 1][1] = intervals[i][1].max(intervals[i+1][1]);
            } else {
                result.push(intervals[i].clone());
            }
            i += 1;
        }
        result.push(intervals[i].clone());

        result
    }
}

fn main() {
    let case_1 = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    // [[1,6],[8,10],[15,18]]
    let case_2 = vec![vec![1, 4], vec![4, 5]];
    // [[1,5]]
    let case_3 = vec![vec![1, 4], vec![5, 6]];
    // [[1,4],[5,6]]

    println!("case_1: {:?}", Solution::merge(case_1.clone()));
    println!("case_2: {:?}", Solution::merge(case_2.clone()));
    println!("case_3: {:?}", Solution::merge(case_3.clone()));

    println!("case_1: {:?}", SolutionAns::merge(case_1.clone()));
    println!("case_2: {:?}", SolutionAns::merge(case_2.clone()));
    println!("case_3: {:?}", SolutionAns::merge(case_3.clone()));

    println!("case_1: {:?}", SolutionAnsCpp::merge(case_1.clone()));
    println!("case_2: {:?}", SolutionAnsCpp::merge(case_2.clone()));
    println!("case_3: {:?}", SolutionAnsCpp::merge(case_3.clone()));
}
