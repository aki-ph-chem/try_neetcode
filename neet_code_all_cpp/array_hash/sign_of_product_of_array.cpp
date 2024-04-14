#include <algorithm>
#include <iostream>
#include <vector>

class Solution {
    public:
        // AC
        int arraySign(std::vector<int>& nums) {
            int result = 1;
            auto sing_func = [](auto x) {
                if(x == 0) {
                    return 0;
                }

                return x / std::max(x, -x);
            };
            std::for_each(nums.begin(), nums.end(), 
                    [&](auto x) 
                    {
                        result *= sing_func(x);
                    });

            return result;
        }
};

// 模範解答より
class SolutionAns {
    public:
        int arraySign(std::vector<int>& nums) {
            int num_neg = 0;
            for(auto& v: nums) {
                if(v == 0) {
                    return 0;
                } else if (v < 0) {
                    ++num_neg;
                }
            }

            if(num_neg & 1) {
                return -1;
            }

            return 1;
        }
};

int main(void) {
    std::vector<int> case_1 = {-1,-2,-3,-4,3,2,1}; 
    // => 1
    std::vector<int> case_2 = {1,5,0,2,-3}; 
    // => 0
    std::vector<int> case_3 = {-1,1,-1,1,-1};
    // => -1

    Solution s_1;

    std::cout << s_1.arraySign(case_1) << std::endl;
    std::cout << s_1.arraySign(case_2) << std::endl;
    std::cout << s_1.arraySign(case_3) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.arraySign(case_1) << std::endl;
    std::cout << s_ans.arraySign(case_2) << std::endl;
    std::cout << s_ans.arraySign(case_3) << std::endl;
}
