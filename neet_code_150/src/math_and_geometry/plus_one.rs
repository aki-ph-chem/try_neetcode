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
}
