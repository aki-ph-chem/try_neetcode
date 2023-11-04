struct Solution {}
impl Solution {
    // inplaceで実装すること
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {}
}

fn main() {
    let mut case_1 = vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        .iter()
        .map(|x| x.to_vec());
    // result
    /*
    [
    [7,4,1],
    [8,5,2],
    [9,6,3]
    ]
         */

    let mut case_2 = vec![
        [5, 1, 9, 11],
        [2, 4, 8, 10],
        [13, 3, 6, 7],
        [15, 14, 12, 16],
    ]
    .iter()
    .map(|x| x.to_vec());

    // result
    /*
        [
    [15,13,2,5],
    [14,3,4,1],
    [12,6,8,9],
    [16,7,10,11]
        ]
             */
}
