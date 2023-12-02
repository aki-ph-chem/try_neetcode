#include <iostream>
#include <vector>

// AC
class Solution {
    public:
        bool can_jump(std::vector<int>& nums) {
            int goal =  nums.size() - 1;

            for(int i = goal - 1; i >= 0; --i) {
                if(i + nums[i] >= goal) {
                    goal = i;
                }
            }

            return goal == 0;
        }
};

// 模範解答
// あんまGreedyっぽくない？ 
class SolutionAns {
    public:
        bool can_jump(std::vector<int>& nums) {
            int n = nums.size();
            int reachable = 0;

            for (int i = 0; i < n; i++) {
                if (i > reachable) {
                    return false;
                }
                reachable = std::max(reachable, i + nums[i]);
                if (reachable >= n - 1) {
                    break;
                }
            }

            return true;
        }
};

int main(void) {
    auto case_1 = std::vector{2, 3, 1, 1, 4};
    auto case_2 = std::vector{3, 2, 1, 0, 4};
    auto case_3 = std::vector{2, 0};
    auto case_4 = std::vector{2,5,0,0};

    Solution s_1;
    std::cout << s_1.can_jump(case_1) << std::endl;
    std::cout << s_1.can_jump(case_2) << std::endl;
    std::cout << s_1.can_jump(case_3) << std::endl;
    std::cout << s_1.can_jump(case_4) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.can_jump(case_1) << std::endl;
    std::cout << s_ans.can_jump(case_2) << std::endl;
    std::cout << s_ans.can_jump(case_3) << std::endl;
    std::cout << s_ans.can_jump(case_4) << std::endl;
}
