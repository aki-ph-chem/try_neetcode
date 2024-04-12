#include <cwchar>
#include <iostream>
#include <utility>
#include <vector>
#include <unordered_map>

class SolutionAns {
    public:
        bool checkSubarraySum(std::vector<int>& nums, int k) {
            std::unordered_map<int,int> map;
            map[0] = -1;
            int sum = 0;

            for(int i = 0; i < nums.size(); ++i) {
                sum += nums[i];
                if(k != 0) {
                    sum %= k;
                }

                if(map.count(sum)) {
                    if(i - map[sum] > 1) {
                        return true;
                    }
                } else {
                    map[sum] = i;
                }
            }

            return false;
        }
};

// Rustの模範解答より
class SolutionAnsRust {
    public:
        // AC
        bool checkSubarraySum(std::vector<int>& nums, int k) {
            std::unordered_map<int,int> map;
            map[0] = -1;
            int sum = 0;

            for(int i = 0; i < nums.size(); ++i) {
                sum += nums[i];
                if(map.count(sum % k)) {
                    if(i - map[sum % k] >= 2) {
                        return true;
                    }
                } else {
                    map.insert({sum % k, i});
                }
            }

            return false;
        }
};

int main(void) {
    using Case = std::pair<std::vector<int>, int>;
    Case case_1 = {{23, 2, 6, 4, 7}, 6};
    // => true
    Case case_2 = {{23, 2, 6, 4, 7}, 13};
    // => false
    Case case_3 = {{1, 0}, 2};
    // => false
    Case case_4 = {{23, 2, 4, 6, 6}, 7};
    // => true
    Case case_5 = {{5, 0, 0, 0}, 3};
    // => true

    SolutionAns s_ans;

    std::cout << s_ans.checkSubarraySum(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.checkSubarraySum(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.checkSubarraySum(case_3.first, case_3.second) << std::endl;
    std::cout << s_ans.checkSubarraySum(case_4.first, case_4.second) << std::endl;
    std::cout << s_ans.checkSubarraySum(case_5.first, case_5.second) << std::endl;

    SolutionAnsRust s_ans_rs;

    std::cout << s_ans_rs.checkSubarraySum(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_rs.checkSubarraySum(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans_rs.checkSubarraySum(case_3.first, case_3.second) << std::endl;
    std::cout << s_ans_rs.checkSubarraySum(case_4.first, case_4.second) << std::endl;
    std::cout << s_ans_rs.checkSubarraySum(case_5.first, case_5.second) << std::endl;

}
