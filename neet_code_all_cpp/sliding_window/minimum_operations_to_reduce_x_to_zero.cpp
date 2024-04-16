#include <algorithm>
#include <iostream>
#include <tuple>
#include <vector>

// Kotlinの模範解答より
class SolutionAnsKotlin {
    public:
        // AC
        int minOperations(std::vector<int>& nums, int x) {
            auto [left, max, current] = std::tuple(0, -1, 0);
            int target = -x; 
            std::for_each(nums.begin(), nums.end(), [&](auto x) { target += x;});

            for(int right = 0; right < nums.size(); ++right) {
                current += nums[right];

                while(left <= right && current > target) {
                    current -= nums[left];
                    ++left;
                }

                if(current == target) {
                    max = std::max(max, right - left + 1);
                }
            }

            if(max == -1) {
                return -1;
            }

            return (int)nums.size() - max;
        }
};

int main(void) {
    using Case = std::pair<std::vector<int>, int>;
    Case case_1 = {{1, 1, 4, 2, 3}, 5};
    // => 2
    Case case_2 = {{5, 6, 7, 8, 9}, 4};
    // => -1
    Case case_3 = {{3, 2, 20, 1, 1, 3}, 10};
    // => 5

    SolutionAnsKotlin s_ans_kt;

    std::cout << s_ans_kt.minOperations(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_kt.minOperations(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans_kt.minOperations(case_3.first, case_3.second) << std::endl;
}
