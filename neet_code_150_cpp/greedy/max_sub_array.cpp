#include <iostream>
#include <vector>

class SolutionAns {
public:
    //Kadaneのアルゴリズム
    int maxSubArray(std::vector<int>& nums) {
        int curr = nums[0];
        int result = nums[0];
        
        for (int i = 1; i < nums.size(); i++) {
            //currをmax(curr + nums[i],nums[i])で更新する
            curr = std::max(curr + nums[i], nums[i]);
            // resultをmax(result, curr)で更新する
            result = std::max(result, curr);
        }
        
        return result;
    }
};

int main(void) {
    auto case_1 = std::vector{-2, 1, -3, 4, -1, 2, 1, -5, 4};
    auto case_2 = std::vector{1};
    auto case_3 = std::vector{5, 4, -1, 7, 8};

    SolutionAns s_ans;
    std::cout << s_ans.maxSubArray(case_1) << std::endl;
    std::cout << s_ans.maxSubArray(case_2) << std::endl;
    std::cout << s_ans.maxSubArray(case_3) << std::endl;
}
