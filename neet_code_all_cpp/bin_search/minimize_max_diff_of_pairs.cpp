#include <iostream>
#include <vector>
#include <algorithm>
#include <climits>

// Kotlinの模範解答より
class SolutionAnsKotlin {
    public:
        // AC
        int minimizeMax(std::vector<int>& nums, int p) {
            if(p == 0) {
                return 0;
            }
            std::sort(nums.begin(), nums.end());
            int n = nums.size();

            auto is_good = [&](int x) {
                int i = 0;
                int count = 0;
                while(i < n - 1) {
                    if(nums[i + 1] - nums[i] <= x) {
                        ++count;
                        i += 2;
                    } else {
                        ++i;
                    }

                    if(count == p) {
                        return true;
                    }
                }

                return false;
            };

            int left = 0, right = INT_MAX;
            while(left < right) {
                int mid = left + (right - left) / 2;
                if(is_good(mid)) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }

            return left;
        }
};

int main(void) {
    using Case = std::pair<std::vector<int>, int>;

    Case case_1 = {{10, 1, 2, 7, 1, 3}, 2};
    Case case_2 = {{4, 2, 1, 2}, 1};

    SolutionAnsKotlin s_ans_kt;

    std::cout << s_ans_kt.minimizeMax(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_kt.minimizeMax(case_2.first, case_2.second) << std::endl;
}
