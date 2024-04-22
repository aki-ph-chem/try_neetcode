#include <cmath>
#include <iostream>
#include <string>
#include <climits>

class SolutionAnsPython {
    public:
        // AC
        int minFlips(std::string s) {
            int n = s.size();
            s += s;
            auto [alt_1, alt_2] = std::pair(std::string{""}, std::string{""});

            for(int i = 0; i < (int)s.size(); ++i) {
                if(i % 2 == 0) {
                    alt_1 += "0";
                    alt_2 += "1";
                } else {
                    alt_1 += "1";
                    alt_2 += "0";
                }
            }

            int result = INT_MAX;
            auto [diff_1, diff_2] = std::pair(0, 0);
            int left = 0;
            for(int right = 0; right < (int)s.size(); ++right) {
                if(s[right] != alt_1[right]) {
                    ++diff_1;
                }
                if(s[right] != alt_2[right]) {
                    ++diff_2;
                }
                if(right - left + 1 > n) {
                    if(s[left] != alt_1[left]) {
                        --diff_1;
                    }
                    if(s[left] != alt_2[left]) {
                        --diff_2;
                    }
                    ++left;
                }

                if(right - left + 1 == n) {
                    result = std::min(result, std::min(diff_1, diff_2));
                }
            }


            return result;
        }
};

int main(void) {
    std::string case_1 = "111000";
    // => 2
    std::string case_2 = "010";
    // => 0
    std::string case_3 = "1110";
    // => 1
    std::string case_4 = "01001001101";
    // => 2

    SolutionAnsPython s_ans_py;
    std::cout << s_ans_py.minFlips(case_1) << std::endl;
    std::cout << s_ans_py.minFlips(case_2) << std::endl;
    std::cout << s_ans_py.minFlips(case_3) << std::endl;
    std::cout << s_ans_py.minFlips(case_4) << std::endl;
}
