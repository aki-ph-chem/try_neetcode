use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

// 解けなかった
struct Solution {}
impl Solution {
    // TLE
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for q in queries {
            let mut size_interval = i32::MAX;
            for v in &intervals {
                if v[0] <= q && q <= v[1] {
                    size_interval = size_interval.min(v[1] - v[0] + 1);
                }
            }

            if size_interval != i32::MAX {
                result.push(size_interval);
            } else {
                result.push(-1);
            }
        }

        result
    }
}

// AC
// C++の模範解答より
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn min_interval(mut intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut sorted_queries = queries.clone();
        let mut pq: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        let mut map: HashMap<i32, i32> = HashMap::new();

        intervals.sort();
        sorted_queries.sort();

        let mut result = vec![];

        let mut i = 0;
        for query in sorted_queries {
            while i < intervals.len() && intervals[i][0] <= query {
                let (left, right) = (intervals[i][0], intervals[i][1]);
                pq.push(Reverse((right - left + 1, right)));
                i += 1;
            }

            while let Some(pq_peek) = pq.peek() {
                if pq_peek.0 .1 >= query {
                    break;
                }
                pq.pop();
            }

            if let Some(pq_peek) = pq.peek() {
                map.insert(query, pq_peek.0 .0);
            } else {
                map.insert(query, -1);
            }
        }

        for q in queries {
            result.push(map[&q]);
        }

        result
    }
}

fn main() {
    let case_1 = (
        vec![vec![1, 4], vec![2, 4], vec![3, 6], vec![4, 4]],
        vec![2, 3, 4, 5],
    );
    // => [3,3,1,4]
    let case_2 = (
        vec![vec![2, 3], vec![2, 5], vec![1, 8], vec![20, 25]],
        vec![2, 19, 5, 22],
    );
    // => [2,-1,4,6]

    /*
    println!(
        "case_1:{:?}",
        Solution::min_interval(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2:{:?}",
        Solution::min_interval(case_2.0.clone(), case_2.1.clone())
    );
    */

    println!(
        "case_1:{:?}",
        SolutionAnsCpp::min_interval(case_1.0.clone(), case_1.1.clone())
    );
    println!(
        "case_2:{:?}",
        SolutionAnsCpp::min_interval(case_2.0.clone(), case_2.1.clone())
    );
}
