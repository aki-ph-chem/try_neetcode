use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn merge_triplets(triplets: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let n = triplets.len();
        let mut set = HashSet::new();

        // targetより大きければmergeしてもtargetにならないので除外
        for i in 0..n {
            if triplets[i][0] > target[0]
                || triplets[i][1] > target[1]
                || triplets[i][2] > target[2]
            {
                continue;
            }

            // そうでないくかつtargetと等しいならばsetに挿入
            for k in 0..3 {
                if triplets[i][k] == target[k] {
                    set.insert(k);
                }
            }
        }

        set.len() == 3
    }
}

fn main() {
    let case_1 = (
        vec![vec![2, 5, 3], vec![1, 8, 4], vec![1, 7, 5]],
        vec![2, 7, 5],
    );
    // true

    let case_2 = (vec![vec![3, 4, 5], vec![4, 5, 6]], vec![3, 2, 5]);
    // false

    let case_3 = (
        vec![vec![2, 5, 3], vec![2, 3, 4], vec![1, 2, 5], vec![5, 2, 3]],
        vec![5, 5, 5],
    );
    //true

    println!("case_1: {}", 
        Solution::merge_triplets(case_1.0.clone(), case_1.1.clone()));
    println!("case_2: {}", 
        Solution::merge_triplets(case_2.0.clone(), case_2.1.clone()));
    println!("case_3: {}", 
        Solution::merge_triplets(case_3.0.clone(), case_3.1.clone()));
}
