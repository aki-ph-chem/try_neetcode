#include <iostream>
#include <vector>
#include <tuple>
#include <algorithm>

// 模範解答
class SolutionAns {
    public:
        int maxFrequency(std::vector<int> nums, int k) {
            std::sort(nums.begin(), nums.end());

            auto [left, right, res] = std::tuple(0, 0, 0);
            long long total = 0;
            int n = nums.size();

            while(right < n) {
                total += nums[right];

                while((long)(right - left + 1) * nums[right] > total + k) {
                    total -= nums[left];
                    ++left;
                }

                res = std::max(res, right - left + 1);
                ++right;
            }

            return res;
        }
};

int main(void) {
    using Case = std::pair<std::vector<int>, int>;

    Case case_1 = {{1, 2, 4}, 5};
    // => 3
    Case case_2 = {{1, 4, 8, 13}, 5};
    // => 2
    Case case_3 = {{3, 9, 6}, 2};
    // => 1

    SolutionAns s_ans;

    std::cout << s_ans.maxFrequency(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.maxFrequency(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.maxFrequency(case_3.first, case_3.second) << std::endl;
}
