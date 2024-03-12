#include <iostream>
#include <vector>

class Solution {
    public:
        // マージソート
        // AC
        std::vector<int> sortArray(std::vector<int> nums) {
            std::vector<int> result = nums;
            auto n = result.size();
            m_sort(result, 0, n);

            return result;
        }

        // クイックソート
        // AC
        std::vector<int> sortArray2(std::vector<int> nums) {
            std::vector<int> result = nums;
            auto n = result.size();
            q_sort(result, 0, n);

            return result;
        }

        //  挿入ソート
        //  TLE
        std::vector<int> sortArray3(std::vector<int> nums) {
            std::vector<int> result = nums;
            auto n = result.size();
            i_sort(result, 0, n);

            return result;
        }

    private:
        void m_sort(std::vector<int>& nums, int left, int right){
            if(right - left == 1){
                return;
            }

            int mid = left + (right - left) / 2;
            m_sort(nums, left, mid);
            m_sort(nums, mid, right);

            // バッファにコピー
            std::vector<int> buff;
            for(int i = left; i < mid; ++i) {
                buff.push_back(nums[i]);
            }
            for(int i = right - 1; i >= mid; --i) {
                buff.push_back(nums[i]);
            }

            // マージ
            auto [idx_left, idx_right] = std::pair(0, buff.size() - 1);
            for(int i = left; i < right; ++i) {
                if(buff[idx_left] <= buff[idx_right]) {
                    nums[i] = buff[idx_left];
                    ++idx_left;
                } else {
                    nums[i] = buff[idx_right];
                    --idx_right;
                }
            }
        }

        void q_sort(std::vector<int>& nums, int left, int right) {
            if(right - left <= 1) {
                return;
            }

            auto pivot_idx = left + (right - left) / 2;
            auto pivot_value = nums[pivot_idx];

            std::swap(nums[pivot_idx], nums[right - 1]);
            auto i = left;
            // nums[j] >= pivot_value => j += 1
            // nums[j] < pivot_value => nums[i], nums[j] をswapして i += 1
            for(int j = left; j < right - 1; ++j) {
                if(nums[j] < pivot_value) {
                    std::swap(nums[i], nums[j]);
                    ++i;
                }
            }

            // 最後にnums[i]とpivot(rightに置いて置いた)を交換
            std::swap(nums[i], nums[right - 1]);

            // 左側、右側をsort
            q_sort(nums, left, i);
            q_sort(nums, i + 1, right);
        }

        void i_sort(std::vector<int>& nums, int left, int right) {
            auto n = (int)nums.size();

            for(int i = 1; i < n; ++i) {
                //v: 挿入する値, j: 挿入する位置
                auto v = nums[i];
                auto j = i;

                while(j > 0) {
                    if(nums[j - 1] > v) {
                        nums[j] = nums[j - 1];
                    } else {
                        break;
                    }

                    --j;
                }

                nums[j] = v;
            }
        }
};

void show_result(std::vector<int>& result) {
    for(auto& v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    std::vector<int> case_1 = {5, 2, 3, 1};
    std::vector<int> case_2 = {5, 1, 1, 2, 0, 0};

    Solution s_1;
    auto res_1 = s_1.sortArray(case_1);
    show_result(res_1);

    auto res_2 = s_1.sortArray(case_2);
    show_result(res_2);

    auto res_1_2 = s_1.sortArray2(case_1);
    show_result(res_1_2);

    auto res_2_2 = s_1.sortArray2(case_2);
    show_result(res_2_2);

    auto res_1_3 = s_1.sortArray3(case_1);
    show_result(res_1_3);

    auto res_2_3 = s_1.sortArray3(case_2);
    show_result(res_2_3);
}
