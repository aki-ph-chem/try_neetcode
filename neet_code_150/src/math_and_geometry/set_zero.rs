struct Solution {}
impl Solution {
    // in place
    pub fn set_zeros(matrix: &mut Vec<Vec<i32>>) {}

    // in placeでない
    pub fn set_zeros_b(matrix: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = matrix.clone(); 

        result
    }

}

fn main() {
    let case_1: Vec<Vec<i32>> = vec![[1, 1, 1], [1, 0, 1], [1, 1, 1]]
        .iter()
        .map(|x| x.to_vec())
        .collect();

    let case_2: Vec<Vec<i32>> = vec![[0,1,2,0],[3,4,5,2],[1,3,1,5]]
        .iter()
        .map(|x| x.to_vec())
        .collect();
}
