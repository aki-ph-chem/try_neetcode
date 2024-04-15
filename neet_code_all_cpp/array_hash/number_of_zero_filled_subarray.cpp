#include <iostream>
#include <vector>

class Solution {
    public:
        // AC
        long long zeroFilledSubarray(std::vector<int>& nums) {
            long long result = 0;

            long long i = 0;
            while(i < nums.size()) {
                if(nums[i] == 0) {
                    long long j = i;
                    while(j < nums.size() && nums[j] == 0) {
                        ++j;
                    }
                    result += (j + 1 - i) * (j - i) / 2;
                    i = j;
                }
                ++i;
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        long long zeroFilledSubarray(std::vector<int>& nums) {
            long long result = 0, count = 0;
            for(int i = 0; i < nums.size(); ++i) {
                if(nums[i]) {
                    result += (count * (count + 1)) /2;
                    count = 0;
                } else {
                    ++count;
                }
            }

            result += (count * (count + 1)) / 2;
            return result;
        }
};

int main(void) {
    std::vector<int> case_1 = {1, 3, 0, 0, 2, 0, 0, 4};
    // => 6
    std::vector<int> case_2 = {0, 0, 0, 2, 0, 0};
    // => 9
    std::vector<int> case_3 = {2, 10, 2019};
    // => 0

    Solution s_1;
    std::cout << s_1.zeroFilledSubarray(case_1) << std::endl;
    std::cout << s_1.zeroFilledSubarray(case_2) << std::endl;
    std::cout << s_1.zeroFilledSubarray(case_3) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.zeroFilledSubarray(case_1) << std::endl;
    std::cout << s_ans.zeroFilledSubarray(case_2) << std::endl;
    std::cout << s_ans.zeroFilledSubarray(case_3) << std::endl;
}
