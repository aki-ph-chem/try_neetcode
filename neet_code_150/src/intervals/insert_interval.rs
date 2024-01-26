// 解けなかった
struct Solution {}
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let new_left = new_interval[0];
        let new_right = new_interval[1];
        let mut result = vec![];

        for i in 1..intervals.len() {
            if new_left <= intervals[i - 1][1] {
                result.push(vec![intervals[i - 1][0], new_right]);
            } else if new_right <= intervals[i][0] {
                result.push(vec![new_left, intervals[i][1]]);
            } else {
                result.push(intervals[i].clone());
            }
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();

        for i in 0..intervals.len() {
            if new_interval[1] < intervals[i][0] {
                res.push(new_interval.clone());
                return [res, intervals[i..].to_vec()].concat().to_vec();
            } else if new_interval[0] > intervals[i][1] {
                res.push(intervals[i].clone());
            } else {
                new_interval = vec![
                    new_interval[0].min(intervals[i][0]),
                    new_interval[1].max(intervals[i][1]),
                ];
            }
        }

        res.push(new_interval);

        res
    }
}

// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let new_start = new_interval[0];
        let new_end = new_interval[1];
        let n = intervals.len();

        for i in 0..n {
            if new_end < intervals[i][0] {
                ans.push(new_interval.clone());
                return [ans, intervals[i..].to_vec()].concat().to_vec();
            } else if intervals[i][1] < new_start {
                ans.push(intervals[i].clone());
            } else {
                new_interval[0] = new_interval[0].min(intervals[i][0]);
                new_interval[1] = new_interval[1].max(intervals[i][1]);
            }
        }

        ans.push(new_interval);
        ans
    }
}

fn main() {
    let case_1 = (vec![vec![1, 3], vec![6, 9]], vec![2, 5]);
    // => [[1, 5],[6, 9]]
    let case_2 = (
        vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ],
        vec![4, 8],
    );
    // => [[1, 2], [3, 10], [12, 16]]
    let case_3 = (vec![vec![1, 2], vec![7, 9]], vec![3, 4]);
    // => [[1,2], [3,4], [7,9]]

    // println!("case_1: {:?}", Solution::insert(case_1.0.clone(), case_1.1.clone()));
    // println!("case_2: {:?}", Solution::insert(case_2.0.clone(), case_2.1.clone()));
    // println!("case_3: {:?}", Solution::insert(case_3.0.clone(), case_3.1.clone()));

    println!("case_1: {:?}", SolutionAns::insert(case_1.0.clone(), case_1.1.clone()));
    println!("case_2: {:?}", SolutionAns::insert(case_2.0.clone(), case_2.1.clone()));
    println!("case_3: {:?}", SolutionAns::insert(case_3.0.clone(), case_3.1.clone()));

    println!("case_1: {:?}", SolutionAnsCpp::insert(case_1.0.clone(), case_1.1.clone()));
    println!("case_2: {:?}", SolutionAnsCpp::insert(case_2.0.clone(), case_2.1.clone()));
    println!("case_3: {:?}", SolutionAnsCpp::insert(case_3.0.clone(), case_3.1.clone()));
}
