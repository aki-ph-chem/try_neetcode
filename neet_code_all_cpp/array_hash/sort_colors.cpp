#include <iostream>
#include <utility>
#include <vector>

class Solution {
    // AC
    public:
        void sortColors(std::vector<int>& nums) {
            auto n = (int)nums.size();
            q_sort(nums, 0, n);
        }

        void q_sort(std::vector<int>& nums, int left, int right) {
            if(right - left <= 1) {
                return;
            }

            auto pivot_idx = left + (right - left) / 2;
            auto pivot_value = nums[pivot_idx];

            std::swap(nums[pivot_idx], nums[right - 1]);
            auto i = left;
            for(int j = left; j < right - 1; ++j) {
                if(nums[j] < pivot_value) {
                    std::swap(nums[i], nums[j]);
                    ++i;
                }
            }

            std::swap(nums[i], nums[right - 1]);

            q_sort(nums, left, i);
            q_sort(nums, i + 1, right);
        }
};

// Rustの模範解答より
class SolutionAnsRust {
    public:
        // AC
        // ラディックスソート
        void sortColors(std::vector<int>& nums) {
            std::vector<int> count(3, 0);
            for(auto& num: nums) {
                ++count[num];
            }

            int i = 0;
            for(int num = 0; num < 3; ++num) {
                auto c = count[num];
                for(int j = 0; j < c; ++j) {
                    nums[i] = num;
                    ++i;
                }
            }
        }
};

// 模範解答 
class SolutionAns {
    public:
        void sortColors(std::vector<int>& nums) {
            int p_1 = 0, p_2 = nums.size() - 1;
            for(int i = p_1; i <= p_2; ++i) {
                // 0だったら左端に詰める
                if(nums[i] == 0) {
                    std::swap(nums[i], nums[p_1]);
                    ++p_1;
                }

                // 2だったら右端に詰める
                if(nums[i] == 2) {
                    std::swap(nums[i], nums[p_2]);
                    --p_2;
                    --i;
                }
            }
        }

        // AC
        void sortColors2(std::vector<int>& nums) {
            int p_1 = 0; 
            int p_2 = nums.size() - 1;

            int i = p_1;
            while(i <= p_2) {
                // 0だったら左端に詰める
                if(nums[i] == 0) {
                    std::swap(nums[i], nums[p_1]);
                    ++p_1;
                }

                // 2だったら右端に詰める
                if(nums[i] == 2) {
                    std::swap(nums[i], nums[p_2]);
                    --p_2;
                    --i;
                }

                ++i;
            }
        }
};

void show_result(std::vector<int>& result) {
    for(auto&v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    std::vector<int> case_1 = {2,0,2,1,1,0};
    // => [0,0,1,1,2,2]
    std::vector<int> case_2 = {2,0,1};
    // => [0,1,2]

    Solution s_1;

    auto res_1 = case_1;
    s_1.sortColors(res_1);
    show_result(res_1);

    auto res_2 = case_2;
    s_1.sortColors(res_2);
    show_result(res_2);

    SolutionAnsRust s_ans_rs;

    auto res_1_ans_rs = case_1;
    s_ans_rs.sortColors(res_1_ans_rs);
    show_result(res_1_ans_rs);

    auto res_2_ans_rs = case_2;
    s_ans_rs.sortColors(res_2_ans_rs);
    show_result(res_2_ans_rs);

    SolutionAns s_ans;

    auto res_1_ans = case_1;
    s_ans.sortColors(res_1_ans);
    show_result(res_1_ans);

    auto res_2_ans = case_2;
    s_ans.sortColors(res_2_ans);
    show_result(res_2_ans);

    auto res_1_ans_2 = case_1;
    s_ans.sortColors2(res_1_ans_2);
    show_result(res_1_ans_2);

    auto res_2_ans_2 = case_2;
    s_ans.sortColors2(res_2_ans_2);
    show_result(res_2_ans_2);
}
