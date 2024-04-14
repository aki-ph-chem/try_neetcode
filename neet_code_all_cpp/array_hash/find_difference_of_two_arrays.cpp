#include <algorithm>
#include <cmath>
#include <iostream>
#include <set>
#include <unordered_set>
#include <vector>

class Solution {
    public:
        // AC
        std::vector<std::vector<int>> findDifference(std::vector<int>& nums1, std::vector<int>& nums2) {
            std::unordered_set<int> set_1, set_2;
            for(auto&v: nums1) {
                set_1.insert(v);
            }
            for(auto&v: nums2) {
                set_2.insert(v);
            }

            std::vector<std::vector<int>> result = {{}, {}};
            for(auto& v: set_1) {
                if(!set_2.count(v)) {
                    result[0].push_back(v);
                }
            }
            for(auto& v: set_2) {
                if(!set_1.count(v)) {
                    result[1].push_back(v);
                }
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::vector<std::vector<int>> findDifference(std::vector<int>& nums1, std::vector<int>& nums2) {
            std::unordered_set<int> set_1(nums1.begin(), nums1.end());
            std::unordered_set<int> set_2(nums2.begin(), nums2.end());

            std::vector<int> list_1, list_2;

            for(auto& n: set_1) {
                if(!set_2.count(n)) {
                    list_1.push_back(n);
                }
            }
            for(auto& n: set_2) {
                if(!set_1.count(n)) {
                    list_2.push_back(n);
                }
            }

            return {list_1, list_2};
        }
};

void show_result(std::vector<std::vector<int>>&& result) {
    std::cout << "res[0]: ";
    for(auto& v: result[0]) {
        std::cout << v << " ";
    }
    std::cout << std::endl;

    std::cout << "res[1]: ";
    for(auto& v: result[1]) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    std::pair<std::vector<int>, std::vector<int>> case_1 = {{1, 2, 3}, {2, 4, 6}};
    std::pair<std::vector<int>, std::vector<int>> case_2 = {{1, 2, 3, 3}, {1, 1, 2, 2}};

    Solution s_1;
    show_result(s_1.findDifference(case_1.first, case_1.second));
    show_result(s_1.findDifference(case_2.first, case_2.second));

    SolutionAns s_ans;
    show_result(s_ans.findDifference(case_1.first, case_1.second));
    show_result(s_ans.findDifference(case_2.first, case_2.second));
}
