#include <algorithm>
#include <exception>
#include <iostream>
#include <vector>

// 模範解答
class SolutionAns {
public:
    int maxProduct(std::vector<int>& nums) {
        int res = nums[0];
        int curMin = 1, curMax = 1;
        
        for(int i = 0; i < nums.size(); i++)
        {
            int n = nums[i];
                
            int tmp = curMax * n;
            curMax = std::max(std::max(n * curMax, n * curMin), n);
            curMin = std::min(std::min(tmp, n * curMin), n);
            res = std::max(res, curMax);
        }
        
        return res;
    }

    // range base forで
    // AC
    int maxProduct2(std::vector<int>& nums) {
        int res = nums[0];
        int curMin = 1, curMax = 1;

        for(const auto &v: nums) {
            int tmp = curMax * v;
            curMax = std::max(std::max(v * curMax, v * curMin), v);
            curMin = std::min(std::min(tmp, v * curMin), v);

            res = std::max(res, curMax);
        }

        return res;
    }

};

int main(void) {
    auto case_1 = std::vector{2, 3, -2, 4};
    // 6
    auto case_2 = std::vector{-2, 0, -1};
    // 0

    SolutionAns s_ans;

    std::cout << s_ans.maxProduct(case_1) << std::endl;
    std::cout << s_ans.maxProduct(case_2) << std::endl;
}
