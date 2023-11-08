struct Solution {}
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res = digits;
        for v in res.iter_mut().rev() {
            *v += 1;
            if *v == 10 {
                *v = 0;
            } else {
                break;
            }
        }

        res
    }
}

fn main() {
}
