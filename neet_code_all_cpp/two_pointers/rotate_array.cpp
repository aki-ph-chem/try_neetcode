#include <iostream>
#include <vector>
#include <algorithm>

class SolutionAns {
    public:
        void rotate(std::vector<int>& nums, int k) {
            k %= nums.size();

            std::reverse(nums.begin(), nums.end());
            std::reverse(nums.begin(), nums.begin() + k);
            std::reverse(nums.begin() + k, nums.end());
        }
};

void show_result(std::vector<int>& result) {
    for(auto&v: result) {
        std::cout << v <<" ";
    }
    std::cout << std::endl;
}

int main(void) {
    using Case = std::pair<std::vector<int>, int>;

    Case case_1 = {{1, 2, 3, 4, 5, 6, 7}, 3};
    // => [5,6,7,1,2,3,4}
    Case case_2 = {{-1, -100, 3, 99}, 2};
    // => [3,99,-1,-100]

    SolutionAns s_ans;

    auto res_1 = case_1.first;
    s_ans.rotate(res_1, case_1.second);
    show_result(res_1);

    auto res_2 = case_2.first;
    s_ans.rotate(res_2, case_2.second);
    show_result(res_2);
}
