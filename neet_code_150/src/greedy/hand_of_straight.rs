// Hand of Straights
//
// group_size: number of group
// hand[i]: numbers of cards
use std::collections::HashMap;

struct Solution {}
impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        // HashMapを使って(数,個数)のリストを作成するところまでは思いついた
        let mut num_list: HashMap<i32, i32> = HashMap::new();
        for n in &hand {
            if !num_list.contains_key(n) {
                num_list.insert(*n, 1);
            }
        }

        true
    }
}

// 模範解答
// C++の模範解答より
// まだ途中
struct SolutionAns {}
impl SolutionAns {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {

        if hand.len() as i32 % group_size != 0 {
            return false;
        }

        let mut num_list: HashMap<i32, i32> = HashMap::new();
        //map
        for n in hand {
            *num_list.entry(n).or_default() += 1;
        }

        while !num_list.is_empty() {
            let current = num_list.iter().next().unwrap().0;
            for i in 0..group_size {
                // current + 0, current + 1,...
                if num_list[&(current + i)] == 0 {
                    return false;
                }

                // 使用した数を削除
            }
        }

        true
    }
}

fn main() {
    let case_1 = (vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3);
    let case_2 = (vec![1, 2, 3, 4, 5], 4);

    Solution::is_n_straight_hand(case_1.0.clone(), 3);
}
