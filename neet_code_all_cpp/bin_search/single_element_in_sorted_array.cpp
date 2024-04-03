#include <iostream>
#include <unordered_map>
#include <vector>

// 縛り: time O(logN), space: O(1)
class Solution {
    public:

        // 縛り無視(1): XORで調べる(time: O(N), space: O(1))
        // AC
        int singleNonDuplicateBit(std::vector<int>& nums) {
            int result = 0;
            for(auto& v: nums) {
                result ^= v;
            }

            return result;
        }

        // 縛り無視(2): mapを使う (time: O(N), space: O(N))
        // AC
        int singleNonDuplicateMap(std::vector<int>& nums) {
            std::unordered_map<int, int> map;
            for(auto& v: nums) {
                ++map[v];
            }

            int result = 0;
            for(auto& [v, n]: map) {
                if(n == 1) {
                    result = v;
                    break;
                }
            }

            return result;
        }
};

// 模範解答 
class SolutionAns {
    public:
        // 縛り通り
        int singleNonDuplicate(std::vector<int>& nums) {
            auto [left, right] = std::pair((int)0, (int)nums.size() - 2);
            while(left <= right) {
                auto mid_0 = left + (right - left) / 2;
                // mid_0が奇数なら1引く、偶数なら1足す
                auto mid_1 = mid_0 ^ 1;

                if(nums[mid_0] == nums[mid_1]) {
                    left = mid_0 + 1;
                } else {
                    right = mid_0 - 1;
                }
            }

            return nums[left];
        }

        // AC
        // XORを使うより２倍遅い
        int singleNonDuplicate2(std::vector<int>& nums) {
            auto [left, right] = std::pair((int)0, (int)nums.size() - 2);
            while(left <= right) {
                auto mid_0 = left + (right - left) / 2;
                // mid_0が奇数なら1引く、偶数なら1足す
                auto mid_1 = 0;
                if(mid_0 & 1) {
                    mid_1 = mid_0 - 1;
                } else {
                    mid_1 = mid_0 + 1;
                }

                if(nums[mid_0] == nums[mid_1]) {
                    left = mid_0 + 1;
                } else {
                    right = mid_0 - 1;
                }
            }

            return nums[left];
        }
};

int main(void) {
    std::vector<int> case_1 =  {1,1,2,3,3,4,4,8,8};
    // => 2
    std::vector<int> case_2 =  {3,3,7,7,10,11,11};
    // => 10

    Solution s_1;

    std::cout << s_1.singleNonDuplicateBit(case_1) << std::endl;
    std::cout << s_1.singleNonDuplicateBit(case_2) << std::endl;

    std::cout << s_1.singleNonDuplicateMap(case_1) << std::endl;
    std::cout << s_1.singleNonDuplicateMap(case_2) << std::endl;

    SolutionAns s_ans;

    std::cout << s_ans.singleNonDuplicate(case_1) << std::endl;
    std::cout << s_ans.singleNonDuplicate(case_2) << std::endl;

    std::cout << s_ans.singleNonDuplicate2(case_1) << std::endl;
    std::cout << s_ans.singleNonDuplicate2(case_2) << std::endl;
}
