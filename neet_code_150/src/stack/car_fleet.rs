// 解けなかった
struct Solution {}
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut delta_p = position.clone();
        for (i, p) in position.iter().enumerate() {
            delta_p[i] = target - p;
        }

        println!("{:?}", delta_p);
        let mut result = 0;

        result
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        //(position, pair)
        let mut position_speed_pair: Vec<(f64, f64)> = position
            .iter()
            .map(|x| *x as f64)
            .zip(speed.iter().map(|x| *x as f64))
            .collect();
        //eprintln!("position_speed_pair: {:?}", position_speed_pair);

        // positionでソート
        position_speed_pair.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        //eprintln!("position_speed_pair: {:?}", position_speed_pair);

        let mut stack = vec![];
        // 後ろにいる車から時間の最大値を探す
        for (pos, speed) in position_speed_pair.iter().rev() {
            stack.push((target as f64 - pos) / speed);
            if stack.len() >= 2 && stack.last() <= stack.get(stack.len() - 2) {
                stack.pop();
            }
        }
        // 後ろから探索したときの最大値以下の時間の集合の個数 = グループ数
        eprintln!("{}", stack.len());
        stack.len() as i32
    }
}

fn main() {
    // target, position, speed
    let case_1 = (12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]); // 3
    let case_2 = (10, vec![3], vec![3]); // 1
    let case_3 = (100, vec![0, 2, 4], vec![4, 2, 1]); // 1

    SolutionAns::car_fleet(case_1.0, case_1.1.clone(), case_1.2.clone());
    SolutionAns::car_fleet(case_2.0, case_2.1.clone(), case_2.2.clone());
    SolutionAns::car_fleet(case_3.0, case_3.1.clone(), case_3.2.clone());
}
