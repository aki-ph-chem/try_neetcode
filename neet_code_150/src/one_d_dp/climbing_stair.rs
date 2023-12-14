// AC
struct Solution {}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut table = vec![0; n as usize + 1];
        if n == 1 {
            return 1;
        }

        if n == 2 {
            return 2;
        }

        if 2 < n {
            table[1] = 1;
            table[2] = 2;
            for i in 3..(n as usize + 1) {
                table[i] = table[i - 1] + table[i - 2];
            }
        }

        table[n as usize]
    }
}

fn main() {
    for i in 1..46 {
        print!("i = {}: ", i);
        println!(" {}", Solution::climb_stairs(i));
    }
}
