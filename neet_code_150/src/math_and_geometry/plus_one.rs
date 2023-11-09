struct Solution {}
impl Solution {
    // AC
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res = digits;
        res.reverse();
        match res.iter_mut().next() {
            Some(v) => *v += 1,
            None => panic!("empty"),
        }

        for i in 0..res.len() {
            if res[i] == 10 {
                res[i] = 0;
                if i + 1 < res.len() {
                    res[i + 1] += 1;
                } else {
                    // 最上位のけたの繰り上がりの処理
                    res.push(1);
                }
            }
        }
        res.reverse();
        res
    }

    // 配列のサイズが大きくなるとオーバーフローしてだめ
    pub fn plus_one_b(digits: Vec<i32>) -> Vec<i32> {
        let mut num = 0;
        for (idx, v) in digits.iter().rev().enumerate() {
            num += v * 10_i32.pow(idx as u32);
        }
        //println!("num: {}", num);
        num += 1;

        let mut res = vec![];
        while num > 0 {
            res.push(num % 10);
            num /= 10;
        }
        res.reverse();

        res
    }

    // AC
    // reverse()が要らない分高速
    pub fn plus_one_c(digits: Vec<i32>) -> Vec<i32> {
        let mut res = digits;
        match res.iter_mut().rev().next() {
            Some(v) => *v += 1,
            None => panic!("empyt"),
        }

        for i in (0..res.len()).rev() {
            if res[i] == 10 {
                res[i] = 0;
                if i as i32 - 1 >= 0 {
                    res[i - 1] += 1;
                } else {
                    res.insert(0, 1);
                }
            }
        }

        res
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut carry: u8 = 1;

        digits = digits
            .into_iter()
            .rev()
            .map(|digit| {
                let val: u8 = digit as u8 + carry;
                carry = val / 10;
                (val % 10) as i32
            })
            .collect();
        digits.reverse();

        if carry == 1 {
            digits.insert(0, 1);
        }
        digits
    }
}

fn main() {
    let case_1 = vec![1, 2, 3]; // [1,2,4]
    let case_2 = vec![4, 3, 2, 1]; // [4,3,2,1]
    let case_3 = vec![9]; // [1,0]
    let case_4 = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]; // [9,8,7,6,5,4,3,2,1,1]
    let case_5 = vec![4, 3, 2, 1, 3, 0];
    let case_6 = vec![9, 9];

    println!("plus_one_b()");
    println!("case_1: {:?}", Solution::plus_one_b(case_1.clone()));
    println!("case_2: {:?}", Solution::plus_one_b(case_2.clone()));
    println!("case_3: {:?}", Solution::plus_one_b(case_3.clone()));
    //println!("case_4: {:?}", Solution::plus_one_b(case_4.clone()));
    println!("case_5: {:?}", Solution::plus_one_b(case_5.clone()));

    println!("plus_one()");
    println!("case_1: {:?}", Solution::plus_one(case_1.clone()));
    println!("case_2: {:?}", Solution::plus_one(case_2.clone()));
    println!("case_3: {:?}", Solution::plus_one(case_3.clone()));
    println!("case_4: {:?}", Solution::plus_one(case_4.clone()));
    println!("case_5: {:?}", Solution::plus_one(case_5.clone()));
    println!("case_6: {:?}", Solution::plus_one(case_6.clone()));

    println!("plus_one_c()");
    println!("case_1: {:?}", Solution::plus_one_c(case_1.clone()));
    println!("case_2: {:?}", Solution::plus_one_c(case_2.clone()));
    println!("case_3: {:?}", Solution::plus_one_c(case_3.clone()));
    println!("case_4: {:?}", Solution::plus_one_c(case_4.clone()));
    println!("case_5: {:?}", Solution::plus_one_c(case_5.clone()));
    println!("case_6: {:?}", Solution::plus_one_c(case_6.clone()));
}
