#include <iostream>
#include <algorithm>
#include <vector>

class Solution {
    public:
        // AC
        int minCostClimbingStairs(std::vector<int>& cost) {
            int n = cost.size();

            // tab_0: 0 スタート, tab_1: 1 スタート 
            std::vector<int> tab_0(n + 1, 1<<30), tab_1(n + 1, 1<<30);
            tab_0[0] = 0;
            tab_1[0] = 0;

            for(int i = 1; i < n + 1; ++i) {
                if(i == 1) {
                    tab_0[i] = cost[0];
                    tab_0[i] = 0;
                } else {
                    tab_0[i] = std::min(tab_0[i - 1] + cost[i - 1],
                            tab_0[i - 2] + cost[i - 2]);
                    tab_1[i] = std::min(tab_1[i - 1] + cost[i - 1],
                            tab_1[i - 2] + cost[i - 2]);
                }
            }

            return std::min(tab_0[n], tab_1[n]);
        }
};

int main(void) {
    auto case_1 = std::vector<int> {10, 15, 20};
    auto case_2 = std::vector<int> {1, 100, 1, 1, 1, 100, 1, 1, 100, 1};

    Solution s_1;

    std::cout << s_1.minCostClimbingStairs(case_1) << std::endl;
    std::cout << s_1.minCostClimbingStairs(case_2) << std::endl;
}
