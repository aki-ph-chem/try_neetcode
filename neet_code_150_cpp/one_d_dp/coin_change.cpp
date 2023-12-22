#include <iostream>
#include <vector>

// 模範解答
class SolutionAns {
    public:
        int coinChange(std::vector<int>& coins, int amount) {
            std::vector<int> dp(amount + 1, amount + 1);
            dp[0] = 0;

            for(int i = 1; i < amount + 1; ++i) {
                for(int j = 0; j < coins.size(); ++j) {
                    if(i - coins[j] >= 0) {
                        // coins[j]を選ばない場合と選ぶ場合をminで比較する
                        dp[i] = std::min(dp[i], dp[i - coins[j]] + 1);
                    }
                }
            }

            // 初期の値のままだったらエラー
            if(dp[amount] == amount + 1) {
                return -1;
            }

            return dp[amount];
        }
};

int main(void) {
    auto case_1 = std::pair(std::vector{1, 2, 5}, 11);
    auto case_2 = std::pair(std::vector{2}, 3);
    auto case_3 = std::pair(std::vector{1}, 0);
    auto case_4 = std::pair(std::vector{1, 100, 500}, 1200);

    SolutionAns s_ans;
    std::cout << s_ans.coinChange(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.coinChange(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.coinChange(case_3.first, case_3.second) << std::endl;
    std::cout << s_ans.coinChange(case_4.first, case_4.second) << std::endl;
}
