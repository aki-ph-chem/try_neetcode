#include <iostream>
#include <vector>
#include <unordered_map>

// 模範解答
class SolutionAns {
    public:
        int subarraySum(std::vector<int>& nums, int k) {
            int n = nums.size();
            int i = 0, j = 0;
            int result = 0;
            int sum = 0;

            std::unordered_map<int, int> map;
            while(j < n) {
                sum += nums[j];
                if(sum == k) {
                    ++result;
                }

                if(map.find(sum - k) != map.end()) {
                    result += map[sum - k];
                }

                ++map[sum];
                ++j;
            } 

            return result;
        }
};

int main(void) {
    using Case = std::pair<std::vector<int>, int>;

    Case case_1 = {{1, 1, 1}, 2};
    // => 2
    Case case_2 = {{1, 2, 3}, 3};
    // => 2
    Case case_3 = {{1}, 0};
    // => 0
    Case case_4 = {{-1, -1, 1}, 0};
    // => 1

    SolutionAns s_ans;

    std::cout << s_ans.subarraySum(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.subarraySum(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.subarraySum(case_3.first, case_3.second) << std::endl;
    std::cout << s_ans.subarraySum(case_4.first, case_4.second) << std::endl;
}
