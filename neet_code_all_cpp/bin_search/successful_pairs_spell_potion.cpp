#include <iostream>
#include <algorithm>
#include <tuple>
#include <vector>

class Solution {
    public:
        // time: O(NlogN)
        // AC
        std::vector<int> successfulPairs(std::vector<int>& spells, std::vector<int>& potions, long long success) {
            std::sort(potions.begin(), potions.end());
            std::vector<int> result;

            for(auto& s: spells) {
                auto [left, right] = std::pair(0, (int)potions.size() - 1);
                int len_success = 0;
                while(left <= right) {
                    int mid = left + (right - left) / 2;
                    if(success <= (long long)potions[mid] * (long long)s) {
                        len_success += right - mid + 1;
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                }
                result.push_back(len_success);
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::vector<int> successfulPairs(std::vector<int>& spells, std::vector<int>& potions, long long success) {
            std::sort(potions.begin(), potions.end());

            int m = potions.size();
            std::vector<int> pairs;

            for(auto& spell: spells) {
                int left = 0, right = m - 1;

                while(left <= right) {
                    int mid = left + (right - left) / 2;
                    long long prod_strength = (long long)potions[mid] * (long long)spell;
                    if(prod_strength < success) {
                        left = mid + 1;
                    } else {
                        right = mid - 1;
                    }
                }

                pairs.push_back(m - left);
            }

            return pairs;
        }
};

void show_result(std::vector<int>& result) {
    for(auto&v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    using Case = std::tuple<std::vector<int>, std::vector<int>, int>;
    Case case_1 = {{5, 1, 3}, {1, 2, 3, 4, 5}, 7};
    // => [4,0,3}
    Case case_2 = {{3, 1, 2}, {8, 5, 8}, 16};
    // => [2,0,2]

    Solution s_1;
    auto res_1 = s_1.successfulPairs(std::get<0>(case_1), std::get<1>(case_1), std::get<2>(case_1));
    show_result(res_1);

    auto res_2 = s_1.successfulPairs(std::get<0>(case_2), std::get<1>(case_2), std::get<2>(case_2));
    show_result(res_2);

    SolutionAns s_ans;
    auto res_1_ans = s_ans.successfulPairs(std::get<0>(case_1), std::get<1>(case_1), std::get<2>(case_1));
    show_result(res_1_ans);

    auto res_2_ans = s_ans.successfulPairs(std::get<0>(case_2), std::get<1>(case_2), std::get<2>(case_2));
    show_result(res_2_ans);
}
