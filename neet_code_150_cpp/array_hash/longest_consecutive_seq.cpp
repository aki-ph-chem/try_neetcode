#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <unordered_set>

class Solution {
    public:
        // Rustの模範解答を変換 -> ジャッジをpassしない 
        int longest_consecutive(const std::vector<int> &nums) {
            std::set<int> set;
            for(const auto &v: nums) {
                set.insert(v);
            }

            int max_cnt = 0;
            for(const auto &n: set) {
                if(set.find(n - 1) != set.end()) {
                    auto next = n + 1;
                    auto cnt = 1;
                    while(set.find(next) != set.end()) {
                        ++cnt;
                        ++next;
                    }
                    max_cnt = std::max(cnt, max_cnt);
                }
            }

            return max_cnt;
        }

        //int longest_consecutive_nlogn(std::vector<int> &nums) {
        // そもそも間違い
        int longest_consecutive_nlogn(std::vector<int> nums) {
            if(nums.size() == 0){
                return 0;
            }
            std::sort(nums.begin(), nums.end());
            int result = 1;
            for(std::size_t i = 1;i < nums.size(); ++i) {
                if(nums[i] - nums[i - 1] == 1) {
                    ++result;
                }
            }
            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        int longest_consecutive(std::vector<int> nums) {
            std::unordered_set<int> s(nums.begin(), nums.end());
            int longest = 0;
            for(auto &n: s){
                //if this is the start of the sequence
                if(!s.count(n - 1)){
                    int length = 1; 
                    while(s.count(n + length))
                        ++length;
                    longest = std::max(longest, length);
                } 
            }
            return longest;
        }
};

int main(void) {
    auto case_1 = std::vector{100, 4, 200, 1, 3, 2};
    auto case_2 = std::vector{100, 8, 20, 10, 5, 2};
    auto case_3 = std::vector{0, 3, 7, 2, 5, 8, 4, 6, 0, 1};

    Solution s_1;
    std::cout << "case_1: " << s_1.longest_consecutive_nlogn(case_1) << std::endl;
    std::cout << "case_2: " << s_1.longest_consecutive_nlogn(case_2) << std::endl;
    std::cout << "case_3: " << s_1.longest_consecutive_nlogn(case_3) << std::endl;

    std::cout << "case_1: " << s_1.longest_consecutive(case_1) << std::endl;
    std::cout << "case_2: " << s_1.longest_consecutive(case_2) << std::endl;
    std::cout << "case_3: " << s_1.longest_consecutive(case_3) << std::endl;
}
