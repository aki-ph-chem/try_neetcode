#include <iostream>
#include <cmath>

class Solution {
    // RustではACなのにC++だとTLEになるのだが
    public:
        // 再帰で解く
        double myPow(double x, int n) {
            if(n >= 0) {
                return pow_raw(x, n);
            } else {
                return 1.0 / pow_raw(x, n);
            }
        }

        // AC
        double myPow2(double x, int n) {
            if(n >= 0) {
                return pow_raw_2(x, n);
            } else {
                return 1.0 / pow_raw_2(x, n);
            }
        }

    private:
        double pow_raw(double x, int n) {
            switch(std::abs(n)) {
                case 0:
                    return 1.0;
                case 1:
                    return x;
                default:
                    if(n % 2 == 0) {
                        return pow_raw(x, n / 2) * pow_raw(x, n / 2);
                    } else {
                        return x * pow_raw(x, n / 2) * pow_raw(x, n / 2);
                    }
            }
        }

        double pow_raw_2(double x, int n) {
            switch(std::abs(n)) {
                case 0:
                    return 1.0;
                case 1:
                    return x;
                default:
                    if(n % 2 == 0) {
                        return std::pow(pow_raw_2(x, n / 2), 2);
                    } else {
                        return x * std::pow(pow_raw_2(x, n / 2), 2);
                    }
            }
        }
};

// 模範解答
class SolutionAns {
    public:
        double myPow(double x, int n) {
            long exponent = abs(n);
            double curr = x;
            double result = 1.0;

            for (long i = exponent; i > 0; i /= 2) {
                if (i % 2 == 1) {
                    result *= curr;
                }
                curr *= curr;
            }

            if (n < 0) {
                return 1.0 / result;
            }
            return result;
        }
};

int main(void) {
    auto case_1 = std::make_pair(2.0000, 10); // 1024.000
    auto case_2 = std::make_pair(2.1000, 3); // 9.26100 
    auto case_3 = std::make_pair(2.0000, -2); // 0.25 
    auto case_4 = std::make_pair(2.0000, 3); // 8
    auto case_5 = std::make_pair(0.00001, 2147483647); // 8

    Solution s_1;
    std::cout << s_1.myPow(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.myPow(case_2.first, case_2.second) << std::endl;
    std::cout << s_1.myPow(case_3.first, case_3.second) << std::endl;
    std::cout << s_1.myPow(case_4.first, case_4.second) << std::endl;
    //std::cout << s_1.myPow(case_5.first, case_5.second) << std::endl;

    SolutionAns s_2;
    std::cout << s_2.myPow(case_1.first, case_1.second) << std::endl;
    std::cout << s_2.myPow(case_2.first, case_2.second) << std::endl;
    std::cout << s_2.myPow(case_3.first, case_3.second) << std::endl;
    std::cout << s_2.myPow(case_4.first, case_4.second) << std::endl;
    std::cout << s_2.myPow(case_5.first, case_5.second) << std::endl;
}
