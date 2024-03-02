struct Solution {}
impl Solution {
    // AC
    // 模範解答は全く同じ
    pub fn reverse_string(s: &mut Vec<char>) {
        let (mut l, mut r) = (0, s.len() - 1);
        while l < r {
            s.swap(l, r);
            l += 1;
            r -= 1;
        }
    }
}
fn main() {
    let case_1 = vec!['h', 'e', 'l', 'l', 'o'];
    // => ['o','l','l','e','h']
    let case_2 = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    // => ['h', 'a', 'n', 'n', 'a', 'H']

    let mut res_1 = case_1.clone();
    Solution::reverse_string(&mut res_1);
    println!("case_1: {:?}", res_1);

    let mut res_2 = case_2.clone();
    Solution::reverse_string(&mut res_2);
    println!("case_2: {:?}", res_2);
}
