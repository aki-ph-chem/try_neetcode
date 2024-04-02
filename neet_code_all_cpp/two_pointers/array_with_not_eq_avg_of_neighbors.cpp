#include <iostream>
#include <tuple>
#include <vector>

class SolutionAns {
    public:
        std::vector<int> rearrangeArray(std::vector<int> nums) {
            for(int i = 1; i < nums.size() - 1; ++i) {
                auto [a, b, c] = std::tuple(nums[i - 1], nums[i], nums[i + 1]);

                if(a > b && b > c || a < b && b < c) {
                    std::swap(nums[i], nums[i + 1]);
                }
            }

            return nums;
        }
};

void show_result(std::vector<int>& res) {
    for(auto& v: res) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    std::vector<int> case_1 = {1, 2, 3, 4, 5};
    // => [1, 2, 4, 5, 3]
    std::vector<int> case_2 = {6, 2, 0, 9, 7};
    // => [9, 7, 6, 2, 0]

    SolutionAns s_ans;
    auto res_1 = s_ans.rearrangeArray(case_1);
    show_result(res_1);
    auto res_2 = s_ans.rearrangeArray(case_2);
    show_result(res_2);
}
