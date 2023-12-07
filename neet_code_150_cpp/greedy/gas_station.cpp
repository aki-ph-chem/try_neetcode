#include <iostream>
#include <vector>

// 初見では解けなかった
// 模範解答
class SolutionAns {
public:
    int canCompleteCircuit(std::vector<int>& gas, std::vector<int>& cost) {
        int n = gas.size();
        
        int totalGas = 0;
        int totalCost = 0;
        // gas,costのtotalを計算
        for (int i = 0; i < n; i++) {
            totalGas += gas[i];
            totalCost += cost[i];
        }
        if (totalGas < totalCost) {
            return -1;
        }
        
        int total = 0;
        int result = 0;
        
        //?
        for (int i = 0; i < n; i++) {
            total += gas[i] - cost[i];
            if (total < 0) {
                total = 0;
                result = i + 1;
            }
        }
        
        return result;
    }
};

class Solution {
    public:
        int canCompleteCircuit(std::vector<int>& gas, std::vector<int> cost) {

            return 1;
        }
};

int main(void) {
    // case_1
    auto gas_case_1 = std::vector{1,2,3,4,5};
    auto cost_case_1 = std::vector{3,4,5,1,2};

    // case_2
    auto gas_case_2 = std::vector{2,3,4};
    auto cost_case_2 = std::vector{3,4,3};

    SolutionAns s_ans;

    std::cout << s_ans.canCompleteCircuit(
            gas_case_1, cost_case_1)
        << std::endl;
    std::cout << s_ans.canCompleteCircuit(
            gas_case_2, cost_case_2)
        << std::endl;
}
