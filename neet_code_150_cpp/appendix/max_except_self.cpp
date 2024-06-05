#include <iostream>
#include <vector>
#include <climits>

//#define DEBUG

template<typename T>
void show_vector(std::vector<T>& vector) {
    for(auto &v: vector) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

class Solution {
    public:
        // O(N^2)
        std::vector<int> product_except_self_sq(const std::vector<int>& nums) {
            auto nums_size = nums.size();
            std::vector<int> result;

            for(int i = 0; i < nums_size; ++i) {
                int max_tmp = -(1<<30);
                for(int j = 0; j < nums_size; ++j) {
                    if(i == j) {
                        continue;
                    }

                    if(max_tmp < nums[j]) {
                        max_tmp = nums[j];
                    }
                }
                result.push_back(max_tmp);
            }
            return result;
        }

        // O(N)
        std::vector<int> product_except_self(const std::vector<int>& nums) {
            // max_list[i]: iまでの要素でのmax
            std::vector<int> max_list(nums.size(), 0);
            max_list[0] = nums[0];
            for(int i = 1; i < (int)nums.size(); ++i) {
                if(max_list[i - 1] <= nums[i]) {
                    max_list[i] = nums[i];
                } else {
                    max_list[i] = max_list[i - 1];
                }
            }

#ifdef DEBUG
            std::cout << "max_list:" << std::endl;
            show_vector(max_list);
#endif

            // result
            std::vector<int> result(nums.size(), 0);
            // resultの最後
            result[result.size() - 1] = max_list[max_list.size() - 2];
            int max_right = nums.back();

            for(int i = (int)nums.size() - 2; 1 <= i; --i) {
                if(max_right <= nums[i + 1]) {
                    max_right = nums[i + 1];
                }

                if(max_list[i - 1] <= max_right) {
                    result[i] = max_right;
                } else {
                    result[i] = max_list[i - 1];
                }
            }
            // resultの最初
            result[0] = max_right;

            return result;
        }
};

// 後から考えた別解
class SolutionLatter {
    public:
        std::vector<int> max_except_self(std::vector<int>& nums) {
            int n = nums.size();
            std::vector<int> prefix(n + 1, INT_MIN);
            for(int i = 0; i < n; ++i) {
                prefix[i + 1] = std::max(nums[i], prefix[i]);
            }

            std::vector<int> result(n, 0);
            int acc = INT_MIN;
            for(int i = n - 1; i >=0; --i) {
                result[i] = std::max(acc, prefix[i]);
                acc = std::max(acc, nums[i]);
            }

            return result;
        }
};

int main(void) {
    auto case_1 = std::vector{1, 2, 3, 4};
    auto case_2 = std::vector{5, 2, 13, 4, 16, 8, 12, 9};

    Solution s_1;
    auto res_1_sq = s_1.product_except_self_sq(case_1);
    show_vector(res_1_sq);
    auto res_2_sq = s_1.product_except_self_sq(case_2);
    show_vector(res_2_sq);

    auto res_1 = s_1.product_except_self(case_1);
    show_vector(res_1);
    auto res_2 = s_1.product_except_self(case_2);
    show_vector(res_2);

    SolutionLatter s_latter;

    auto res_1_latter = s_latter.max_except_self(case_1);
    auto res_2_latter = s_latter.max_except_self(case_2);

    show_vector(res_1_latter);
    show_vector(res_2_latter);
}
