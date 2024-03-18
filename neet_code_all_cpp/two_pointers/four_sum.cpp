#include <iostream>
#include <vector>
#include <algorithm>

// 模範解答
class SolutionAns {
    public:
        std::vector<std::vector<int>> fourSum(std::vector<int>& nums, int target) {
            std::sort(nums.begin(), nums.end());

            std::vector<std::vector<int> > res;
            int n = nums.size();

            for (int i = 0; i < n; i++) {
                if (i > 0 && nums[i] == nums[i - 1])
                    continue;
                for (int j = i + 1; j < n; j++) {
                    if (j > (i + 1) && nums[j] == nums[j - 1])
                        continue;

                    auto [l, r] = std::pair(j + 1, n - 1);
                    while (l < r) {
                        long sm = (long)nums[i] + (long)nums[j] + (long)nums[l] + (long)nums[r];
                        if (sm == target) {
                            res.push_back(std::vector<int>{nums[i], nums[j], nums[l], nums[r]});
                            ++l;
                            while (l < r && nums[l] == nums[l - 1])
                                ++l;
                        }
                        else if (sm > target)
                            --r;
                        else
                            ++l;
                    }
                }
            }
            return res;
        }
};

void show_result(std::vector<std::vector<int>>& result) {
    for(auto& v: result) {
        for(auto& w: v) {
            std::cout <<w << " ";
        }
        std::cout << std::endl;
    }
}

int main(void) {
    using Case = std::pair<std::vector<int>, int>;

    Case case_1 = {{1, 0, -1, 0, -2, 2}, 0};
    // =>  [[-2,-1,1,2},[-2,0,0,2],[-1,0,0,1]]
    Case case_2 = {{2, 2, 2, 2, 2}, 8};
    // => [[2,2,2,2]]

    SolutionAns s_ans;

    auto res_1 = s_ans.fourSum(case_1.first, case_1.second);
    show_result(res_1);

    auto res_2 = s_ans.fourSum(case_2.first, case_2.second);
    show_result(res_2);
}
