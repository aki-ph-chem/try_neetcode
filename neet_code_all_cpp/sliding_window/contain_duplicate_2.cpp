#include <iostream>
#include <utility>
#include <vector>
#include <unordered_map>
#include <unordered_set>
#include <climits>
#include <algorithm>

class Solution {
    public:
        // AC
        bool containsNearbyDuplicate(std::vector<int>& nums, int k) {
            std::unordered_map<int, std::vector<int>> map;

            // {value: index}なmapを作る
            for(int i = 0; i < (int)nums.size(); ++i) {
                if(map.find(nums[i]) != map.end()) {
                    map[nums[i]].push_back((int)i);
                } else {
                    map.insert({nums[i], std::vector{(int)i}});
                }
            }

            int min_diff = INT_MAX;
            for(auto& [v, idxs]: map) {
                if(idxs.size() > 1) {
                    std::sort(idxs.begin(), idxs.end());
                    for(int i = 0; i < (int)idxs.size() - 1; ++i) {
                        min_diff = std::min(min_diff, idxs[i + 1] - idxs[i]);
                    }
                }
            }

            return min_diff <= k;
        }
};

// 模範解答
class SolutionAns {
    public:
        bool containsNearbyDuplicate(std::vector<int>& nums, int k) {
            std::unordered_map<int, int> number_map;

            for(int i = 0; i < nums.size(); ++i) {
                auto num = nums[i];

                if(number_map.find(num) != number_map.end() && i - number_map[num] <= k) {
                    return true;
                } else {
                    number_map[num] = i;
                }
            }

            return false;
        }
};

// AC
// Rustの模範解答より
class SolutionAnsRust {
    public:
        bool containsNearbyDuplicate(std::vector<int>& nums, int k) {
            std::unordered_set<int> window;
            int l = 0;

            for(int r = 0; r < nums.size(); ++r) {
                if(r - l > k) {
                    window.erase(nums[l]);
                    ++l;
                }

                if(window.find(nums[r]) != window.end()) {
                    return true;
                }

                window.insert(nums[r]);
            }

            return false;
        }
};

int main(void) {
    using Case = std::pair<std::vector<int>, int>;

    Case case_1 = {{1,2,3,1}, 3};
    // => true
    Case case_2 = {{1,0,1,1}, 1};
    // => true
    Case case_3 = {{1,2,3,1,2,3}, 2};
    // => false
    Case case_4 = {{1} ,1};
    // => false

    Solution s_1;
    std::cout << s_1.containsNearbyDuplicate(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.containsNearbyDuplicate(case_2.first, case_2.second) << std::endl;
    std::cout << s_1.containsNearbyDuplicate(case_3.first, case_3.second) << std::endl;
    std::cout << s_1.containsNearbyDuplicate(case_4.first, case_4.second) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.containsNearbyDuplicate(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.containsNearbyDuplicate(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.containsNearbyDuplicate(case_3.first, case_3.second) << std::endl;
    std::cout << s_ans.containsNearbyDuplicate(case_4.first, case_4.second) << std::endl;

    SolutionAnsRust s_rs;
    std::cout << s_rs.containsNearbyDuplicate(case_1.first, case_1.second) << std::endl;
    std::cout << s_rs.containsNearbyDuplicate(case_2.first, case_2.second) << std::endl;
    std::cout << s_rs.containsNearbyDuplicate(case_3.first, case_3.second) << std::endl;
    std::cout << s_rs.containsNearbyDuplicate(case_4.first, case_4.second) << std::endl;
}
