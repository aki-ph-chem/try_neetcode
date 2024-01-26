//縛り: 64ビット整数(符号あり、符号なし)をストアしておくことはできないとする
struct Solution {}
impl Solution {
    // 縛りを守った実装
    pub fn reverse(mut x: i32) -> i32 {
        let sign = if x >= 0 { 1 } else { -1 };
        x *= sign;

        let mut num_list = vec![];
        while x > 0 {
            num_list.push(x % 10);
            x /= 10;
        }

        let mut result = 0;
        for (i, v) in num_list.iter().rev().enumerate() {
            let tmp = *v * 10_i32.pow(i as u32);
            result += tmp;
        }

        result
    }

    // 縛り無視ならこう実装する
    // AC
    pub fn reverse_anyway(mut x: i32) -> i32 {
        let sign = if x >= 0 { 1 } else { -1 };
        x *= sign;

        let mut num_list = vec![];
        while x > 0 {
            num_list.push(x % 10);
            x /= 10;
        }

        let mut result = 0_i64;
        for (i, v) in num_list.iter().rev().enumerate() {
            result += (*v as i64) * 10_i64.pow(i as u32);
        }

        if result >= i32::MAX as i64 {
            return 0;
        }

        sign * (result as i32)
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn reverse(x: i32) -> i32 {
        let mut res = 0;
        let mut x = x;
        while x != 0 {
            let digit = x % 10;
            x /= 10;

            if res > std::i32::MAX / 10 || (res == std::i32::MAX / 10 && digit > std::i32::MAX % 10) {
                return 0;
            }
            if res < std::i32::MIN / 10 || (res == std::i32::MIN / 10 && digit < std::i32::MIN % 10) {
                return 0;
            }

            res = (res * 10) + digit;
        }
        res
    }
}

fn main() {
    let case_1 = 123;
    // => 321
    let case_2 = -123;
    // => -321
    let case_3 = 120;
    // => 21

    println!("case_1: {}", Solution::reverse_anyway(case_1));
    println!("case_2: {}", Solution::reverse_anyway(case_2));
    println!("case_3: {}", Solution::reverse_anyway(case_3));

    println!("case_1: {}", SolutionAns::reverse(case_1));
    println!("case_2: {}", SolutionAns::reverse(case_2));
    println!("case_3: {}", SolutionAns::reverse(case_3));
}
