struct Solution {}
impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, mut amount: i32) -> i32 {
        let mut result = 0;
        coins.sort();

        for v in coins.iter().rev() {
            result += amount / v;
            amount = amount % v;
        }

        result
    }
}

fn main() {
    let case_1 = (vec![1, 2, 5], 11);
    // 5 + 5 + 1 => 3 
    let case_2 = (vec![2], 3);
    // -1
    let case_3 = (vec![1], 0);
    // 0
}
