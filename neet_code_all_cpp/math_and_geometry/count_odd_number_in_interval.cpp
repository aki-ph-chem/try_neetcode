#include <iostream>

class Solution {
    public:
        // AC
        int countOdds(int low, int high) {
            int num = high - low + 1;
            if(num % 2 == 0) {
                return num / 2;
            }

            if(low % 2 == 0) {
                return num / 2;
            } else {
                return num / 2 + 1;
            }
        }

        // AC
        int countOdds2(int low, int high) {
            int num = high - low + 1;
            if(num % 2 == 0 || low % 2 == 0) {
                return num / 2;
            }

            return num / 2 + 1;
        }
};

// Kotlinの模範解答より
class SolutionAnsKotlin {
    public:
        // AC
        int countOdds1(int low, int high) {
            int result = 0;
            result += (high - low) / 2;
            if(low % 2 == 1 || high % 2 == 1) {
                ++result;
            }

            return result;
        }

        // AC
        int countOdds2(int low, int high) {
            return (high + 1) / 2 - low / 2;
        }
};

int main(void) {
    std::pair<int, int> case_1 = {3, 7};
    std::pair<int, int> case_2 = {8, 10};

    Solution s_1;

    std::cout << s_1.countOdds(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.countOdds(case_2.first, case_2.second) << std::endl;

    std::cout << s_1.countOdds2(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.countOdds2(case_2.first, case_2.second) << std::endl;

    SolutionAnsKotlin s_ans_kt;

    std::cout << s_ans_kt.countOdds1(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_kt.countOdds1(case_2.first, case_2.second) << std::endl;

    std::cout << s_ans_kt.countOdds2(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_kt.countOdds2(case_2.first, case_2.second) << std::endl;
}
