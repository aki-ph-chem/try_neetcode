#include <iostream>
#include <vector>

class SolutionAns {
    public:
        std::vector<std::vector<int>> permute(std::vector<int>& nums) {
            std::vector<std::vector<int>> result;
            dfs(nums, 0, result);

            return result;
        }

    private:
        void dfs(std::vector<int>& nums, int start, std::vector<std::vector<int>>& result) {
            if(start == nums.size()) {
                result.push_back(nums);
                return;
            }

            for(int i = start; i < nums.size(); ++i) {
                std::swap(nums[i], nums[start]);
                dfs(nums, start + 1, result);
                std::swap(nums[i], nums[start]);
            }
        }
};

void show_result(const std::vector<std::vector<int>>& result) {
    for(const auto& v: result) {
        for(const auto& w: v) {
            std::cout << w << " ";
        }
        std::cout << '\n';
    }
}

int main(void) {
    auto case_1 = std::vector{1, 2, 3};
    auto case_2 = std::vector{0, 1};

    SolutionAns s_ans;
    auto res_1_ans = s_ans.permute(case_1);
    auto res_2_ans = s_ans.permute(case_2);

    show_result(res_1_ans);
    show_result(res_2_ans);
}
