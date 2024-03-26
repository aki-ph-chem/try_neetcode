use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

// 解けなかった
struct Solution {}
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let n = fruits.len();
        let mut map: HashMap<i32, i32> = HashMap::new();
        for v in fruits {
            *map.entry(v).or_default() += 1;
        }

        let mut buckets = vec![-1; n + 1];
        for (key, v) in map {
            buckets[v as usize] = key;
        }
        //println!("buckets: {:?}", buckets);

        let mut limits = 2;
        let mut result = 0;
        for (idx, v) in buckets.iter().enumerate().rev() {
            if limits > 0 && *v != -1 {
                limits -= 1;
                result += idx as i32;
            } else {
                continue;
            }
        }

        result
    }

    pub fn total_fruit_2(fruits: Vec<i32>) -> i32 {
        let n = fruits.len();
        let mut dq: VecDeque<i32> = VecDeque::new();
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut result = 0;

        for v in fruits {
            dq.push_back(v);
            *map.entry(v).or_default() += 1;

            if map.len() <= 2 {}
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let (mut left, mut total) = (0, 0);
        let mut result = 0;

        for f in &fruits {
            *map.entry(*f).or_default() += 1;
            total += 1;

            while map.len() > 2 {
                let f_left = fruits[left];
                match map.remove(&f_left) {
                    Some(v) if v > 1 => {
                        map.insert(f_left, v - 1);
                    }

                    _ => {}
                }

                total -= 1;
                left += 1;
            }

            result = result.max(total);
        }

        result
    }
}

// fruits[i]: i番目の木が実らせることができるフルーツの種類
// 持てるバスケットは二個
// 一つのバスケットは１種類のフルーツのみ入れられる
// 一つのバスケットに容量制限はない
// バスケット(二個)に収まりきらない木があったらそこで停止

fn main() {
    let case_1 = vec![1, 2, 1];
    // => 3
    let case_2 = vec![0, 1, 2, 2];
    // => 3
    let case_3 = vec![1, 2, 3, 2, 2];
    // => 4
    let case_4 = vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4];
    // => 5

    /*
    println!("case_1: {:?}", Solution::total_fruit(case_1.clone()));
    println!("case_2: {:?}", Solution::total_fruit(case_2.clone()));
    println!("case_3: {:?}", Solution::total_fruit(case_3.clone()));
    println!("case_4: {:?}", Solution::total_fruit(case_4.clone()));
    */

    println!("case_1: {:?}", SolutionAns::total_fruit(case_1.clone()));
    println!("case_2: {:?}", SolutionAns::total_fruit(case_2.clone()));
    println!("case_3: {:?}", SolutionAns::total_fruit(case_3.clone()));
    println!("case_4: {:?}", SolutionAns::total_fruit(case_4.clone()));
}
