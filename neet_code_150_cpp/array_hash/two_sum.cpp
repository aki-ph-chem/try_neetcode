#include <iostream>
#include <vector>
#include <unordered_map>

class Solution {
    public:
        // O(N^2)
        std::vector<int> two_sum(const std::vector<int>& nums, int target) {
            std::vector<int> result;
            for(std::size_t i = 0; i < nums.size(); ++i) {
                for(std::size_t j = i+1; j < nums.size(); ++j) {
                    if(target == nums[i] + nums[j]) {
                        result.push_back(i);
                        result.push_back(j);
                        break;
                    }
                }
            }
            return result;
        }

        // O(N^2)より良いオーダーで書きたい
        std::vector<int> two_sum_2(const std::vector<int>& nums, int target) {
            std::unordered_map<int, int> map;
            std::vector<int> result;
            for(std::size_t i = 0; i < nums.size(); ++i) {
                auto diff = target - nums[i];
                if(map.find(diff) != map.end()) {
                }
            }

            return result;
        }
};

int main(void) {
    auto nums_1 = std::vector{2,7,11,15};
    auto target_1 = 9;

    auto nums_2 = std::vector{3,2,4};
    auto target_2 = 6;

    auto nums_3 = std::vector{3,3};
    auto target_3 = 6;

    Solution s_1;
    for(const auto &v: s_1.two_sum(nums_1, target_1)) {
        std::cout << v <<" ";
    }
    std::cout << std::endl;
    for(const auto &v: s_1.two_sum(nums_2, target_2)) {
        std::cout << v <<" ";
    }
    std::cout << std::endl;
    for(const auto &v: s_1.two_sum(nums_3, target_3)) {
        std::cout << v <<" ";
    }
    std::cout << std::endl;
}
