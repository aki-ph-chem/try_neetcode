use std::collections::{HashMap, HashSet};

struct Solution {}
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut set: HashSet<i32> = HashSet::new();
        let mut result = vec![];

        for edge in &edges {
            if set.contains(&edge[1]) {
                result.push(edge[0]);
                result.push(edge[1]);
            } else {
                set.insert(edge[1]);
            }
        }

        result
    }
}

// AC
// C++の模範解答より
// Union Findを使う
struct SolutionAnsCpp {}
impl SolutionAnsCpp {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();

        let mut parents = vec![];
        let mut ranks = vec![];

        for i in 0..=(n as i32) {
            parents.push(i);
            ranks.push(i);
        }

        let mut result = vec![];
        for edg in edges {
            let (n_1, n_2) = (edg[0], edg[1]);

            if !Self::do_union(&mut parents, &mut ranks, n_1, n_2) {
                result = vec![n_1, n_2];
                break;
            }
        }

        result
    }

    // Union-Findのクエリの実装
    fn do_find(parents: &mut Vec<i32>, n: i32) -> i32 {
        let mut p = parents[n as usize];

        while p != parents[p as usize] {
            parents[p as usize] = parents[parents[p as usize] as usize];
            p = parents[p as usize];
        }

        p
    }

    fn do_union(parents: &mut Vec<i32>, ranks: &mut Vec<i32>, n_1: i32, n_2: i32) -> bool {
        let (p_1, p_2) = (Self::do_find(parents, n_1), Self::do_find(parents, n_2));

        if p_1 == p_2 {
            return false;
        }

        if ranks[p_1 as usize] > ranks[p_2 as usize] {
            parents[p_2 as usize] = p_1;
            ranks[p_1 as usize] += ranks[p_2 as usize];
        } else {
            parents[p_1 as usize] = p_2;
            ranks[p_2 as usize] += ranks[p_1 as usize];
        }

        true
    }
}

fn main() {
    let case_1 = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    // => [2, 3]
    let case_2 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]];
    // => [1, 4]
    let case_3 = vec![vec![1, 3], vec![3, 4], vec![1, 5], vec![3, 5], vec![2, 3]];
    // => [3, 5]

    /*
    println!(
        "case_1: {:?}",
        Solution::find_redundant_connection(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        Solution::find_redundant_connection(case_2.clone())
    );
    */

    println!(
        "case_1: {:?}",
        SolutionAnsCpp::find_redundant_connection(case_1.clone())
    );
    println!(
        "case_2: {:?}",
        SolutionAnsCpp::find_redundant_connection(case_2.clone())
    );
}
