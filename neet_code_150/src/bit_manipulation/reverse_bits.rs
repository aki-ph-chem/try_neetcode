// まだ途中
struct Solution {}
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut result = 0;
        for i in (0..32).rev() {
            if x & (1 << i) != 0 {
                result += 1;
            } else {
            }
        }

        result
    }
}

fn main() {
    let case_1 = 0b00000010100101000001111010011100_u32;
    // => 0b00111001011110000010100101000000 (964176192)
    let case_2 = 0b11111111111111111111111111111101_u32;
    // => 0b10111111111111111111111111111111 (3221225471)
    let case_3 = 0b1101;
    // => 0b1011
    
    println!("{:032b}", 0b1101);
    println!("{:032b}", 0b1011);
}
