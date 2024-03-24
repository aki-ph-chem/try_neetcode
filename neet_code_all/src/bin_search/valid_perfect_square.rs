// AC
struct Solution {}
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 1 {
            return true;
        }

        let num = num as i64;

        let (mut left, mut right) = (0, num);

        while left <= right {
            let mid = left + (right - left) / 2;

            if mid.pow(2) == num {
                return true;
            }

            if mid.pow(2) < num {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        false
    }
}

fn main() {
    let case_1 = 16;
    // => true
    let case_2 = 14;
    // => false
    let case_3 = 2147483647;
    // => false

    println!("case_1: {}", Solution::is_perfect_square(case_1));
    println!("case_2: {}", Solution::is_perfect_square(case_2));
    println!("case_3: {}", Solution::is_perfect_square(case_3));
}
