#include <iostream>
#include <vector>
#include <climits>
#include <tuple>

class Solution {
    public:
        // AC
        int bestClosingTime(std::string customers) {
            auto n = customers.size();
            std::vector<int> prefix_sum_left(n + 1, 0), prefix_sum_right(n + 1, 0);

            auto p_state = [&](auto i, auto c) {
                if(customers[i] == c) {
                    return 1;
                }

                return 0;
            };

            for(int i = 0; i < n; ++i) {
                prefix_sum_left[i + 1] = prefix_sum_left[i] + p_state(i, 'N');
                prefix_sum_right[n - 1 - i] = prefix_sum_right[n - 1 - i + 1] + p_state(n - 1 - i, 'Y');
            }

            auto [idx_penalty, penalty] = std::pair(0, INT_MAX);
            for(int i = 0; i < prefix_sum_left.size(); ++i) {
                if(prefix_sum_left[i] + prefix_sum_right[i] < penalty) {
                    penalty = prefix_sum_left[i] + prefix_sum_right[i];
                    idx_penalty = i;
                }
            }

            return idx_penalty;
        }
};

// 模範解答
class SolutionAns {
    public:
        int bestClosingTime(std::string customers) {
            int result = -1, idx_max = 0, penalty = 0;
            for(int i = 0; i < customers.size(); ++i) {
                if(customers[i] == 'Y') {
                    ++penalty;
                } else {
                    --penalty;
                }

                if(penalty > idx_max) {
                    idx_max = penalty;
                    result = i;
                }
            }

            return ++result;
        }
};

// Kotlinの模範解答より
class SolutionAnsKotlin {
    public:
        // AC
        int bestClosingTime(std::string customers) {
            auto [current, max, close_time] = std::tuple(0, 0, 0);
            for(int i = 0; i < customers.size(); ++i) {
                if(customers[i] == 'Y') {
                    ++current;
                } else {
                    --current;
                }

                if(current > max) {
                    max = current;
                    close_time = i + 1;
                }
            }

            return close_time;
        }
};

int main(void) {
    std::string case_1 = "YYNY";
    // => 2
    std::string case_2 = "NNNNN";
    // => 0
    std::string case_3 = "YYYY";
    // => 4

    Solution s_1;
    std::cout << s_1.bestClosingTime(case_1) << std::endl;
    std::cout << s_1.bestClosingTime(case_2) << std::endl;
    std::cout << s_1.bestClosingTime(case_3) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.bestClosingTime(case_1) << std::endl;
    std::cout << s_ans.bestClosingTime(case_2) << std::endl;
    std::cout << s_ans.bestClosingTime(case_3) << std::endl;

    SolutionAnsKotlin s_ans_kt;
    std::cout << s_ans_kt.bestClosingTime(case_1) << std::endl;
    std::cout << s_ans_kt.bestClosingTime(case_2) << std::endl;
    std::cout << s_ans_kt.bestClosingTime(case_3) << std::endl;
}
