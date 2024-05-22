// hit 1: 命令を一通り実行したあとの座標を計算してみよう
// hit 2: 経路が閉じるのは最終状態の座標が(0, 0) になっているときもしくは移動方向が(0,1)
struct Solution;
impl Solution {
    // AC
    pub fn is_robot_bounded(instructions: String) -> bool {
        let (mut x, mut y) = (0, 0);
        let (mut d_x, mut d_y) = (0, 1);

        for c in instructions.chars() {
            match c {
                'G' => {
                    (x, y) = (x + d_x, y + d_y);
                }
                'L' => {
                    let tmp = d_x;
                    d_x = d_y;
                    d_y = -tmp;
                }
                _ => {
                    let tmp = d_x;
                    d_x = -d_y;
                    d_y = tmp;
                }
            }
            //println!("x, y, d: {x}, {y}, {d_x}, {d_y}");
        }

        //println!("x, y: {x}, {y}");

        if (x == 0 && y == 0) || (d_x != 0 || d_y != 1) {
            true
        } else {
            false
        }
    }
}

fn main() {
    let case_1 = "GGLLGG".to_string();
    // => true
    let case_2 = "GG".to_string();
    // => false
    let case_3 = "GL".to_string();
    // => true

    println!("case_1: {}", Solution::is_robot_bounded(case_1.clone()));
    println!("case_2: {}", Solution::is_robot_bounded(case_2.clone()));
    println!("case_3: {}", Solution::is_robot_bounded(case_3.clone()));
}
