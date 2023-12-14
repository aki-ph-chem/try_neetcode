#include <iostream>
#include <vector>
#include <algorithm>

#define DEBUG

// 模範解答
class Solution {
public:
    int rob(std::vector<int>& nums) {
        int prev = 0;
        int curr = 0;
        int next = 0;
        
        for (int i = 0; i < nums.size(); i++) {
            next = std::max(prev + nums[i], curr);
            prev = curr;
            curr = next;

#ifdef DEBUG
            std::cout <<"prev, curr, next: " << prev << " ," <<  curr << ", " << next << std::endl;
#endif
        }
        
        return curr;
    }
};

int main(void) {
    auto case_1 = std::vector{1, 2, 3, 1};
    auto case_2 = std::vector{2, 7, 9, 3, 1};

    Solution s_ans;

    std::cout << s_ans.rob(case_1) << std::endl;
    std::cout << s_ans.rob(case_2) << std::endl;
}
