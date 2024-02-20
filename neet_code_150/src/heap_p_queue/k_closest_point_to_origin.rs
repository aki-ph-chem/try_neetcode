use std::cmp::Reverse;
use std::collections::BinaryHeap;

// 解けなかった
struct Solution {}
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let l2_norm_sq = |p: &Vec<i32>| p[0].pow(2) + p[1].pow(2);

        let mut p_heap: BinaryHeap<(Reverse<i32>, Vec<i32>)> = BinaryHeap::new();
        for p in &points {
            let dist_p = l2_norm_sq(p);
            p_heap.push((Reverse(dist_p), p.clone()));
        }

        /*
        println!("p_heap: {:?}", p_heap);
        for p in &p_heap {
            println!("p: {:?}", p);
        }
        */

        let mut result = vec![];
        while (p_heap.len() as i32) > k {
            let p = p_heap.peek().unwrap().1.clone();
            result.push(p);
            p_heap.pop();
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut pts = BinaryHeap::new();
        for point in points {
            let dist = point[0].pow(2) + point[1].pow(2);
            pts.push(Reverse((dist, point[0], point[1])));
        }

        let mut res = vec![];
        for i in 0..k {
            match pts.pop() {
                Some(Reverse((dist, x, y))) => res.push(vec![x, y]),
                None => {}
            }
        }

        res
    }
}

fn main() {
    let case_1 = (vec![vec![1, 3], vec![-2, 2]], 1);
    // => [[-2,2]]
    let case_2 = (vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2);
    // => [[3,3],[-2,4]]

    /*
    println!(
        "case_1: {:?}",
        Solution::k_closest(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        Solution::k_closest(case_2.0.clone(), case_2.1)
    );
    */

    println!(
        "case_1: {:?}",
        SolutionAns::k_closest(case_1.0.clone(), case_1.1)
    );
    println!(
        "case_2: {:?}",
        SolutionAns::k_closest(case_2.0.clone(), case_2.1)
    );
}
