#include <iostream>
#include <vector>

class Solution {
    public:
    // Rustの別解より
    int change(int amount ,std::vector<int>& coins) {
        // dp[i]: i - 1までのスコア
        std::vector<int> dp(amount + 1, 0);
        dp[0] = 1;

        for(int i = 0;i < coins.size(); ++i) {
            for(int j = coins[i]; j <= amount; ++j) {
                dp[j] += dp[j - coins[i]];
            }
        }
        return dp[amount];
    }

    int change_2(int amount ,std::vector<int>& coins) {
        std::vector<int> dp(amount + 1, 0);
        dp[0] = 1;

        for(const auto & c: coins) {
            for(int i = c; i <= amount; ++i) {
                dp[i] += dp[i - c];  
            }
        }

        return dp[amount];
    }
};

int main(void) {
    auto case_1 = std::pair(5, std::vector{1,2,5});
    // 5 = 5
    // 5 = 2 + 2 + 1
    // 5 = 2 + 1 + 1 + 1
    // 5 = 1 + 1 + 1 + 1 + 1
    // => 4
    auto case_2 = std::pair(3, std::vector{2});
    // 0
    auto case_3 = std::pair(10, std::vector{10});
    // 1

    Solution s_1;

    std::cout << s_1.change(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.change(case_2.first, case_2.second) << std::endl;
    std::cout << s_1.change(case_3.first, case_3.second) << std::endl;

    std::cout << s_1.change_2(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.change_2(case_2.first, case_2.second) << std::endl;
    std::cout << s_1.change_2(case_3.first, case_3.second) << std::endl;
}
