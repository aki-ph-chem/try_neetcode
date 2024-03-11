#include <iostream>
#include <vector>
#include <unordered_map>
#include <algorithm>

class Solution {
    public:
        // AC
        int majorityElement(std::vector<int>& nums) {
            auto n = nums.size();
            std::unordered_map<int,int> map;

            for(auto& v: nums) {
                ++map[v];
            }

            auto majo_idx = nums[0];
            for(auto& [idx, num]: map) {
                if(num > n / 2) {
                    majo_idx = idx;
                    break;
                }
            }

            return majo_idx;
        }

        // AC
        int majorityElement2(std::vector<int>& nums) {
            auto n = nums.size();
            std::unordered_map<int,int> map;

            auto majo_idx = nums[0];
            for(auto& v: nums) {
                ++map[v];
                if(map.find(v) != map.end()) {
                    if(map[v] > n / 2) {
                        majo_idx = v;
                        break;
                    }
                }
            }

            return majo_idx;
        }
};

// Rustの模範解答より
class SolutionAnsRust {
    public:
        // time: O(N), space: O(1)
        // Boyer-Moore algorithm
        // AC
        int majorityElement(std::vector<int>& nums) {
            auto [res, count] = std::pair(0, 0);

            for(auto& n : nums) {
                if(count == 0) {
                    res = n;
                }

                if(n == res) {
                    ++count;
                } else {
                    --count;
                }
            }

            return res;
        }

        // AC
        // time: O(N), space: O(n)
        int majorityElement2(std::vector<int>& nums) {
            std::unordered_map<int, int> map;
            auto [res, max_count] = std::pair(0, 0);

            for(auto& num: nums) {
                ++map[num];

                if(map.find(num) != map.end()) {
                    if(map[num] > max_count) {
                        res = num;
                    }
                }

                max_count = std::max(map[num], max_count);
            }

            return res;
        }

        // AC
        // time: O(NlogN), space: O(1)
        int majorityElement3(std::vector<int> nums) {
            std::sort(nums.begin(), nums.end());
            return nums[nums.size() / 2];
        }
};

// 模範解答
class SolutionAns {
    public:
        int majorityElement(std::vector<int>& nums) {
            auto [count, res] = std::pair(0, 0);

            for(auto& num: nums) {
                if(!count) {
                    res = num;
                }

                count += (num == res) ? 1: -1;
            }

            return res;
        }
};

int main(void) {
    std::vector<int> case_1 = {3, 2, 3};
    // => 3
    std::vector<int> case_2 = {2,2,1,1,1,2,2};
    // => 2

    Solution s_1;
    std::cout << s_1.majorityElement(case_1) << std::endl;
    std::cout << s_1.majorityElement(case_2) << std::endl;

    std::cout << s_1.majorityElement2(case_1) << std::endl;
    std::cout << s_1.majorityElement2(case_2) << std::endl;

    SolutionAnsRust s_ans_rs;
    std::cout << s_ans_rs.majorityElement(case_1) << std::endl;
    std::cout << s_ans_rs.majorityElement(case_2) << std::endl;

    std::cout << s_ans_rs.majorityElement2(case_1) << std::endl;
    std::cout << s_ans_rs.majorityElement2(case_2) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.majorityElement(case_1) << std::endl;
    std::cout << s_ans.majorityElement(case_2) << std::endl;
} 
