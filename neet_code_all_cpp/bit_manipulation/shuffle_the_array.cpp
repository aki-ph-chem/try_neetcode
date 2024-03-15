#include <iostream>
#include <vector>

class Solution {
    public:
        std::vector<int> shuffle(std::vector<int>& nums, int n) {
            std::vector<int>  result;

            auto [i,j] = std::pair(0, n);
            while(i < n && j < 2 *n) {
                result.push_back(nums[i]);
                ++i;
                result.push_back(nums[j]);
                ++j;
            }

            return result;
        }
};

// AC
// Cの模範解答より
class SolutionC {
    public:
        std::vector<int> shuffle(std::vector<int>& nums, int n) {
            std::vector<int> result(nums.size(), 0);

            auto [i, k] = std::pair(0, 0);

            while(i < n) {
                result[k++] = nums[i];
                result[k++] = nums[i + n];
                ++i;
            }

            return result;
        }
};

void show_result(std::vector<int>& result) {
    for(auto& v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(){
    using Case = std::pair<std::vector<int>, int>;
    Case case_1 = {{2, 5, 1, 3, 4, 7}, 3};
    // => [2,3,5,4,1,7]
    Case case_2 = {{1, 2, 3, 4, 4, 3, 2, 1}, 4};
    // => [1,4,2,3,3,2,4,1]
    Case case_3 = {{1, 1, 2, 2}, 2};
    // => [1,2,1,2]

    Solution s_1;
    auto res_1 = s_1.shuffle(case_1.first, case_1.second);
    show_result(res_1);

    auto res_2 = s_1.shuffle(case_2.first, case_2.second);
    show_result(res_2);

    auto res_3 = s_1.shuffle(case_3.first, case_3.second);
    show_result(res_3);

    SolutionC s_ans_c;

    auto res_1_c = s_ans_c.shuffle(case_1.first, case_1.second);
    show_result(res_1_c);

    auto res_2_c = s_ans_c.shuffle(case_2.first, case_2.second);
    show_result(res_2_c);

    auto res_3_c = s_ans_c.shuffle(case_3.first, case_3.second);
    show_result(res_3_c);

}
