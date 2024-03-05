// 解けなかった
struct Solution {}
impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        let len = flowerbed.len();

        if len == 1 {
            if flowerbed[0] == 0 {
                return n - 1 == 0;
            }
            return n == 0;
        }

        for i in 0..len {
            if i == 0 {
                if flowerbed[i] == 0 && flowerbed[i + 1] == 0 {
                    flowerbed[i] = 1;
                    n -= 1;
                }
            } else if i == len - 1 {
                if flowerbed[i] == 0 && flowerbed[i - 1] == 0 {
                    flowerbed[i] = 1;
                    n -= 1;
                }
            } else {
                if flowerbed[i - 1] == 0 && flowerbed[i] == 0 && flowerbed[i + 1] == 0 {
                    flowerbed[i] = 1;
                    n -= 1;
                }
            }

            if n == 0 {
                println!("flowerbed: {:?}", flowerbed);
                return true;
            }
        }

        println!("flowerbed: {:?}", flowerbed);
        n == 0
    }

    pub fn can_place_flowers_2(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        let len = flowerbed.len();

        if len == 1 {
            if flowerbed[0] == 0 {
                return n - 1 == 0;
            }
            return n == 0;
        }

        let mut l = 0;
        while l < len && n != 0 {
            if l == 0 {
                if flowerbed[l] == 0 && flowerbed[l + 1] == 0 {
                    flowerbed[l] = 1;
                    n -= 1;
                }
            } else if l == len - 1 {
                if flowerbed[l] == 0 && flowerbed[l - 1] == 0 {
                    flowerbed[l] = 1;
                    n -= 1;
                }
            } else {
                if flowerbed[l - 1] == 0 && flowerbed[l] == 0 && flowerbed[l + 1] == 0 {
                    flowerbed[l] = 1;
                    n -= 1;
                }
            }

            l += 1;
        }

        n == 0
    }
}

// 模範解答
struct SolutionAns {}
impl SolutionAns {
    // time O(N), space: O(1)
    pub fn can_place_flowers_1(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut empty = if flowerbed[0] != 0 { 0 } else { 1 };
        let mut n = n;

        for f in flowerbed {
            if f != 0 {
                n -= (empty - 1) / 2;
                empty = 0;
            } else {
                empty += 1;
            }
        }

        n -= empty / 2;

        n <= 0
    }

    // time O(N), space: O(1)
    pub fn can_place_flowers_2(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        for i in 0..flowerbed.len() {
            if n == 0 {
                return true;
            }

            if (i == 0 || flowerbed[i - 1] == 0)
                && flowerbed[i] == 0
                && (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0)
            {
                flowerbed[i] = 1;
                n -= 1;
            }
        }

        n == 0
    }

    // time O(N), space: O(N)
    pub fn can_place_flowers_3(flowerbed: Vec<i32>, mut n: i32) -> bool {
        let mut f: Vec<i32> = vec![vec![0], flowerbed, vec![0]]
            .into_iter()
            .flatten()
            .collect();

        for i in 1..f.len() - 1 {
            if f[i - 1] == 0 && f[i] == 0 && f[i + 1] == 0 {
                f[i] = 1;
                n -= 1;
            }
        }

        n <= 0
    }
}

fn main() {
    let case_1 = (vec![1, 0, 0, 0, 1], 1);
    // => true
    let case_2 = (vec![1, 0, 0, 0, 1], 2);
    // => false
    let case_3 = (vec![1, 0, 0, 0, 0, 1], 2);
    // => false
    let case_4 = (vec![0, 0, 1, 0, 1], 1);
    // => true
    let case_5 = (vec![1, 0, 0, 0, 1, 0, 0], 2);
    // => true
    let case_6 = (vec![0, 0, 1, 0, 0], 1);
    // => true

    println!(
        "case_3: {:?}",
        SolutionAns::can_place_flowers_1(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_3: {:?}",
        SolutionAns::can_place_flowers_2(case_3.0.clone(), case_3.1)
    );
    println!(
        "case_3: {:?}",
        SolutionAns::can_place_flowers_3(case_3.0.clone(), case_3.1)
    );

    println!(
        "case_4: {:?}",
        SolutionAns::can_place_flowers_1(case_4.0.clone(), case_4.1)
    );
    println!(
        "case_4: {:?}",
        SolutionAns::can_place_flowers_2(case_4.0.clone(), case_4.1)
    );
    println!(
        "case_4: {:?}",
        SolutionAns::can_place_flowers_3(case_4.0.clone(), case_4.1)
    );

    println!(
        "case_5: {:?}",
        SolutionAns::can_place_flowers_1(case_5.0.clone(), case_5.1)
    );
    println!(
        "case_5: {:?}",
        SolutionAns::can_place_flowers_2(case_5.0.clone(), case_5.1)
    );
    println!(
        "case_5: {:?}",
        SolutionAns::can_place_flowers_3(case_5.0.clone(), case_5.1)
    );
}
