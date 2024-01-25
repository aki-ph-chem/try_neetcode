// AC 
struct Solution {}
impl Solution {
    // AC
    pub fn reverse_bits(x: u32) -> u32 {
        let mut result = 0;
        let mut bit_list = vec![];

        for i in 0..32 {
            if x & (1 << i) == 0 {
                bit_list.push(0);
            } else {
                bit_list.push(1);
            }
        }

        for (i, v) in bit_list.iter().rev().enumerate() {
            result += v * (1 << i);
        }

        result
    }

    // AC
    pub fn reverse_bits_2(x: u32) -> u32 {
        let mut result = 0;

        for (a, b) in (0..32).rev().zip(0..32) {
            if x & (1 << a) != 0 {
                result += 1 << b;
            }
        }

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn reverse_bits(mut x: u32) -> u32 {
        (0..32).fold(0, |mut res, _| {
            res = (res << 1) | (x & 1);
            x >>= 1;
            res
        })
    }
}

fn main() {
    let case_1 = 0b00000010100101000001111010011100_u32;
    // => 0b00111001011110000010100101000000 (964176192)
    let case_2 = 0b11111111111111111111111111111101_u32;
    // => 0b10111111111111111111111111111111 (3221225471)
    let case_3 = 0b1101;
    let case_4 = 0b10;
    
    println!("cse_1: {:032b}", Solution::reverse_bits(case_1));
    println!("cse_2: {:032b}", Solution::reverse_bits(case_2));
    println!("cse_3: {:032b}", Solution::reverse_bits(case_3));
    println!("cse_4: {:032b}", Solution::reverse_bits(case_4));

    println!("cse_1: {:032b}", Solution::reverse_bits_2(case_1));
    println!("cse_2: {:032b}", Solution::reverse_bits_2(case_2));
    println!("cse_3: {:032b}", Solution::reverse_bits_2(case_3));
    println!("cse_4: {:032b}", Solution::reverse_bits_2(case_4));

    println!("cse_1: {:032b}", SolutionAns::reverse_bits(case_1));
    println!("cse_2: {:032b}", SolutionAns::reverse_bits(case_2));
    println!("cse_3: {:032b}", SolutionAns::reverse_bits(case_3));
    println!("cse_4: {:032b}", SolutionAns::reverse_bits(case_4));
}
