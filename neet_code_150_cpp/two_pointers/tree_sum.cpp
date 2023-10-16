#include <iostream>
#include <vector>
#include <algorithm>
#include <unordered_set>

class Solution {
    public:
        // O(N^2): 同じ要素を二回使ってしまう(set周りが悪い) 
        std::vector<std::vector<int>> three_sum(std::vector<int>& nums) {

            std::unordered_set<int> set;
            set.insert(nums.begin(), nums.end());
            std::vector<std::vector<int>> result;

            for(std::size_t i = 0; i < nums.size(); ++i) {
                for(std::size_t j = i + 1; j < nums.size(); ++j) {

                    int diff = -nums[i] - nums[j];
                    if(set.find(diff) != set.end()) {
                        result.push_back(std::vector{nums[i], nums[j], diff});
                    }
                }
            }
            return result;
        }

        // O(N^3): まだ重複する: 配列に同じ整数が含まれているため
        std::vector<std::vector<int>> three_sum_cu(std::vector<int>& nums) {
            std::vector<std::vector<int>> result;

            for(std::size_t i = 0; i < nums.size(); ++i) {
                for(std::size_t j = i + 1; j < nums.size(); ++j) {
                    for(std::size_t k = j + 1; k < nums.size(); ++k) {
                        if(nums[i] + nums[j] + nums[k] == 0) {
                        result.push_back(std::vector{nums[i], nums[j], nums[k]});
                        }
                    }
                }
            }
            return result;
        }
};

class SolutionAns {
    public:
        std::vector<std::vector<int>> threeSum(std::vector<int>& nums) {
            std::vector<std::vector<int>> result;

            int n = nums.size();
            if (n < 3) {
                return result;
            }

            std::sort(nums.begin(), nums.end());

            for (int i = 0; i < n - 2; i++) {
                if (nums[i] > 0) {
                    break;
                }
                if (i > 0 && nums[i - 1] == nums[i]) {
                    continue;
                }

                int j = i + 1;
                int k = n - 1;

                while (j < k) {
                    int sum = nums[i] + nums[j] + nums[k];

                    if (sum < 0) {
                        j++;
                    } else if (sum > 0) {
                        k--;
                    } else {
                        result.push_back({nums[i], nums[j], nums[k]});

                        while (j < k && nums[j] == nums[j + 1]) {
                            j++;
                        }
                        j++;

                        while (j < k && nums[k - 1] == nums[k]) {
                            k--;
                        }
                        k--;
                    }
                }
            }

            return result;
        }
};

void show_result(std::vector<std::vector<int>>&& result) {
    for(const auto &v: result) {
        for(const auto &w: v) {
            std::cout << w << " ";
        }
        std::cout << std::endl;
    }
}

int main(void) {
    auto case_1 = std::vector{-1, 0, 1, 2, -1, -4};
    auto case_2 = std::vector{0, 1, 1};
    auto case_3 = std::vector{0, 0, 0};

    Solution s_1;
    show_result(s_1.three_sum_cu(case_1));
    show_result(s_1.three_sum_cu(case_2));
    show_result(s_1.three_sum_cu(case_3));

    SolutionAns s_2;
    show_result(s_2.threeSum(case_1));
    show_result(s_2.threeSum(case_2));
    show_result(s_2.threeSum(case_3));
}
