// C++の模範解答より
struct Solution {}
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        // その文字が最後に何番目にあったかを記録
        let mut last_idx = vec![0; 26];
        for (idx, c) in s.chars().enumerate() {
            last_idx[c as usize - 'a' as usize] = idx;
        }

        let (mut size , mut end) = (0, 0);
        let mut result = vec![];

        for (idx, c) in s.chars().enumerate() {
            size += 1;
            end = std::cmp::max(end, last_idx[c as usize - 'a' as usize]);
            if idx == end {
                result.push(size);
                size = 0;
            }
        }

        result
    }
}

fn main() {
    let case_1 = "ababcbacadefegdehijhklij".to_string();
    // [9, 7, 8]
    let case_2 = "eccbbbbdec".to_string();
    // [10]

    println!("case_1: {:?}", Solution::partition_labels(case_1));
    println!("case_1: {:?}", Solution::partition_labels(case_2));
}
