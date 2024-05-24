#include <iostream>
#include <unordered_map>
#include <array>

// AC
class ParkingSystem {
    private:
        int max_big;
        int max_medium;
        int max_small;
        int n_big;
        int n_medium;
        int n_small;

    public:
        ParkingSystem(int big, int medium, int small)
            :max_big(big), max_medium(medium), max_small(small),
            n_big(0), n_medium(0), n_small(0)
        {}

        bool addCar(int carType) {
            switch(carType) {
                case 1:
                    {
                        if(n_big >= max_big) {
                            return false; 
                        } else {
                            ++n_big;
                            break;
                        }
                    }
                case 2:
                    {
                        if(n_medium >= max_medium) {
                            return false; 
                        } else {
                            ++n_medium;
                            break;
                        }
                    }

                default:
                    {
                        if(n_small >= max_small) {
                            return false; 
                        } else {
                            ++n_small;
                            break;
                        }
                    }
            }

            return true;
        }
};

// AC
class ParkingSystemAnsPython {
    private:
        std::unordered_map<int, std::array<int, 2>> parking;

    public:
        ParkingSystemAnsPython(int big, int medium, int small) {
            parking = {
                {1, {0, big}},
                {2, {0, medium}},
                {3, {0, small}}
            };
        }

        bool addCar(int carType) {
            if(parking[carType][0] + 1 <= parking[carType][1]) {
                ++parking[carType][0];
                return true;
            }

            return false;
        }
};

int main(void) {
    ParkingSystem p_0(1, 1, 0);
    std::cout << p_0.addCar(1) << std::endl; 
    std::cout << p_0.addCar(2) << std::endl; 
    std::cout << p_0.addCar(3) << std::endl; 
    std::cout << p_0.addCar(1) << std::endl; 

    ParkingSystemAnsPython p_py(1, 1, 0);
    std::cout << p_py.addCar(1) << std::endl; 
    std::cout << p_py.addCar(2) << std::endl; 
    std::cout << p_py.addCar(3) << std::endl; 
    std::cout << p_py.addCar(1) << std::endl; 
}
