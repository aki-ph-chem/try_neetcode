/*

n(= gas.len()): number of gas station
gas[i]        : amount of ga at the i th station
const[i]      : travel from (i) th station to (i + 1) th station
(const[i - 1]      : travel from (i - 1) th station to (i) th station)

*/

pub struct Solution {}
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut max_sub, mut max_idx) = (-1 << 30, 0);
        for i in 0..gas.len() {
            if max_sub < gas[i] + cost[i] {
                max_sub = gas[i] + cost[i];
                max_idx = i;
            }
        }
        println!("({}, {})", gas[max_idx], cost[max_idx]);

        1
    }

    pub fn can_complete_sq(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        for i in 0..n {
            if Solution::start_from_k(gas.clone(), cost.clone(), i) {
                return i as i32;
            }
        }
        -1
    }

    pub fn circle_idx(num: &Vec<i32>, idx_ini: usize) {
        let n = num.len();
        let mut idx_current = idx_ini;

        for _i in 0..n {
            println!("idx: {}, v: {}", idx_current, num[idx_current]);

            if idx_current == n - 1 {
                idx_current = 0;
            } else {
                idx_current += 1;
            }
        }
    }

    // kからスタートして完走できるか
    pub fn start_from_k(gas: Vec<i32>, cost: Vec<i32>, k: usize) -> bool {
        let n = gas.len();
        let mut gas_tot = 0;
        let mut idx_current = k;

        gas_tot += gas[k];
        for _i in 1..n {
            println!("gas_tot = {}", gas_tot);

            gas_tot += if idx_current == 0{
                gas[idx_current] - cost[n - 1]
            } else {
                gas[idx_current] - cost[idx_current - 1]
            };

            if gas_tot < 0 {
                println!("stop at ({}) th station", idx_current - 1);
                return false;
            }

            if idx_current == n - 1 {
                idx_current = 0;
            } else {
                idx_current += 1;
            }
        }

        println!("gas_tot = {}", gas_tot);
        println!("stop at ({}) th station", gas.len() - 1);

        true
    }
}

fn main() {
    let case_1 = (vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]);
    let case_2 = (vec![2, 3, 4], vec![3, 4, 3]);

    println!(
        "{}",
        Solution::start_from_k(case_1.0.clone(), case_1.1.clone(), 0)
    );
    println!(
        "{}",
        Solution::start_from_k(case_1.0.clone(), case_1.1.clone(), 3)
    );

    /*
    let a_0 = vec![1,2,3,4,5];
    Solution::circle_idx(&a_0, 0);
    Solution::circle_idx(&a_0, 2);
    Solution::circle_idx(&a_0, 3);
    */
}
