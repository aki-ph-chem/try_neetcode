use std::collections::HashMap;

// AC
struct ParkingSystem {
    max_big: i32,
    n_big: i32,
    max_medium: i32,
    n_medium: i32,
    max_small: i32,
    n_small: i32,
}

impl ParkingSystem {
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            max_big: big,
            max_medium: medium,
            max_small: small,
            n_big: 0,
            n_medium: 0,
            n_small: 0,
        }
    }

    pub fn add_car(&mut self, car_type: i32) -> bool {
        match car_type {
            1 => {
                if self.n_big >= self.max_big {
                    return false;
                }
                self.n_big += 1;
            }
            2 => {
                if self.n_medium >= self.max_medium {
                    return false;
                }
                self.n_medium += 1;
            }
            _ => {
                if self.n_small >= self.max_small {
                    return false;
                }
                self.n_small += 1;
            }
        }

        true
    }
}

// Pyrhonの模範解答より
// AC
struct SolutionAnsPython {
    parking: HashMap<i32, [i32; 2]>,
}

impl SolutionAnsPython {
    pub fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            parking: HashMap::from([(1, [0, big]), (2, [0, medium]), (3, [0, small])]),
        }
    }

    pub fn add_car(&mut self, car_type: i32) -> bool {
        if self.parking[&car_type][0] + 1 <= self.parking[&car_type][1] {
            if let Some(num_car) = self.parking.get_mut(&car_type) {
                num_car[0] += 1;
            }
            return true;
        }

        false
    }
}

fn main() {
    let mut p_0 = ParkingSystem::new(1, 1, 0);
    println!("p_0.add_car(1): {}", p_0.add_car(1));
    // => true
    println!("p_0.add_car(2): {}", p_0.add_car(2));
    // => true
    println!("p_0.add_car(3): {}", p_0.add_car(3));
    // => false
    println!("p_0.add_car(1): {}", p_0.add_car(1));
    // => false

    let mut p_py = SolutionAnsPython::new(1, 1, 0);
    println!("p_py.add_car(1): {}", p_py.add_car(1));
    // => true
    println!("p_py.add_car(2): {}", p_py.add_car(2));
    // => true
    println!("p_py.add_car(3): {}", p_py.add_car(3));
    // => false
    println!("p_py.add_car(1): {}", p_py.add_car(1));
    // => false
}
