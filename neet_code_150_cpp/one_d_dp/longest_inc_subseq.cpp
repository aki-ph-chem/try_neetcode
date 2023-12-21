#include <iostream>
#include <iterator>
#include <vector>
#include <algorithm>

// 模範解答
class SolutionAns {
    public:
        // 同じアルゴリズムでもRustより一桁ほど遅い
        int lengthOfLIS(std::vector<int>& nums) {
            int n = nums.size();
            std::vector<int> dp(n, 1);

            int result = 1;

            for (int i = 1; i < n; i++) {
                for (int j = 0; j < i; j++) {
                    if (nums[j] < nums[i]) {
                        dp[i] = std::max(dp[i], dp[j] + 1);
                    }
                }
                result = std::max(result, dp[i]);
            }

            return result;
        }

        // AC
        // Rustの模範解答より
        // std::lower_bound()を使って二分探索をする
        int lengthOfLIS2(std::vector<int>& nums) {
            std::vector<int> tails;

            for(const auto &v: nums) {
                // nums[idx] >= v となる最小のidxを二分探索で探索する
                auto position = std::lower_bound(tails.begin(), tails.end(), v); 
                std::size_t idx = std::distance(tails.begin(), position);

                if (idx == tails.size()) {
                    tails.push_back(v);
                } else {
                    tails[idx] = v;
                }
            }

            return (int) tails.size();
        }
};

int main(void) {
    auto case_1 = std::vector{10, 9, 2, 5, 3, 7, 101, 18};
    // 4
    auto case_2 = std::vector{0, 1, 0, 3, 2, 3};
    // 4
    auto case_3 = std::vector{7, 7, 7, 7, 7, 7, 7};
    // 1

    SolutionAns s_ans;
    std::cout << s_ans.lengthOfLIS(case_1) << std::endl;
    std::cout << s_ans.lengthOfLIS(case_2) << std::endl;
    std::cout << s_ans.lengthOfLIS(case_3) << std::endl;

    std::cout << s_ans.lengthOfLIS2(case_1) << std::endl;
    std::cout << s_ans.lengthOfLIS2(case_2) << std::endl;
    std::cout << s_ans.lengthOfLIS2(case_3) << std::endl;
}
