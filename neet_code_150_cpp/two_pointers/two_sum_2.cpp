#include <iostream>
#include <vector>

class Solution {
    public:
        // O(NlongN)
        // ソート済みの配列 -> 二分探索
        std::vector<int> two_sum(const std::vector<int>& numbers, int target) {
            for(std::size_t i = 0; i < numbers.size(); ++i) {
                int diff = target - numbers[i];
                int idx_l = i + 1;
                int idx_r = numbers.size() - 1;

                while(idx_l <= idx_r) {
                    int mid = (idx_l + idx_r) / 2;
                    if(diff ==  numbers[mid]) {
                        return std::vector{static_cast<int>(i + 1), mid + 1};
                    }

                    if(diff < numbers[mid]) {
                        idx_r = mid - 1;
                    } else {
                        idx_l = mid + 1;
                    }
                }
            }
            return std::vector{-1};
        } 

        //O(N^2)
        // 流石にTLEした
        std::vector<int> two_sum_sq(const std::vector<int>& numbers, int target) {
            for(std::size_t i = 0; i < numbers.size(); ++i) {
                for(std::size_t j = i + 1; j < numbers.size(); ++j) {
                    if(target == numbers[i] + numbers[j]) {
                        return std::vector{static_cast<int>(i + 1), static_cast<int>(j + 1)};
                    }
                }
            }
            return std::vector{-1};
        }

        // Rustの模範解答を変換
        std::vector<int> two_sum_rust(const std::vector<int>& numbers, int target) {
            std::size_t idx_l = 0, idx_r = numbers.size() - 1;
            while(idx_l < idx_r) {
                if(numbers[idx_l] + numbers[idx_r] == target) {
                    return  std::vector{static_cast<int>(idx_l + 1), static_cast<int>(idx_r + 1)};
                } else if (numbers[idx_l] + numbers[idx_r] < target) {
                    ++idx_l;
                } else {
                    --idx_r;
                }
            }
            return std::vector{-1};
        }
};

// 模範解答
class SolutionAns{
    public:
        std::vector<int> twoSum(std::vector<int>& numbers, int target) {
            int i = 0;
            int j = numbers.size() - 1;

            std::vector<int> result;

            while (i < j) {
                int sum = numbers[i] + numbers[j];
                if (sum < target) {
                    i++;
                } else if (sum > target) {
                    j--;
                } else {
                    result.push_back(i + 1);
                    result.push_back(j + 1);
                    break;
                }
            }

            return result;
        }
};

int main(void) {
    auto case_1_array = std::vector{2,7,11,15};
    auto case_1_target = 9;
    auto case_2_array = std::vector{2,3,4};
    auto case_2_target = 6;
    auto case_3_array = std::vector{-1,0};
    auto case_3_target = -1;

    Solution s_1;
    std::cout << "O(nlogn)" << std::endl;
    for(const auto &v: s_1.two_sum(case_1_array, case_1_target)) {
        std::cout << v << " "; 
    }
    std::cout << std::endl;
    for(const auto &v: s_1.two_sum(case_2_array, case_2_target)) {
        std::cout << v << " "; 
    }
    std::cout << std::endl;
    for(const auto &v: s_1.two_sum(case_3_array, case_3_target)) {
        std::cout << v << " "; 
    }
    std::cout << std::endl;

    std::cout << "O(n^2)" << std::endl;
    for(const auto &v: s_1.two_sum_sq(case_1_array, case_1_target)) {
        std::cout << v << " "; 
    }
    std::cout << std::endl;
    for(const auto &v: s_1.two_sum_sq(case_2_array, case_2_target)) {
        std::cout << v << " "; 
    }
    std::cout << std::endl;
    for(const auto &v: s_1.two_sum_sq(case_3_array, case_3_target)) {
        std::cout << v << " "; 
    }
    std::cout << std::endl;

    std::cout << "rust" << std::endl;
    for(const auto &v: s_1.two_sum_rust(case_1_array, case_1_target)) {
        std::cout << v << " "; 
    }
    std::cout << std::endl;
    for(const auto &v: s_1.two_sum_rust(case_2_array, case_2_target)) {
        std::cout << v << " "; 
    }
    std::cout << std::endl;
    for(const auto &v: s_1.two_sum_rust(case_3_array, case_3_target)) {
        std::cout << v << " "; 
    }
    std::cout << std::endl;

    SolutionAns s_2;
    std::cout << "模範解答" << std::endl;
    for(const auto &v: s_2.twoSum(case_1_array, case_1_target)) {
        std::cout << v << " "; 
    }
    std::cout << std::endl;
    for(const auto &v: s_2.twoSum(case_2_array, case_2_target)) {
        std::cout << v << " "; 
    }
    std::cout << std::endl;
    for(const auto &v: s_2.twoSum(case_3_array, case_3_target)) {
        std::cout << v << " "; 
    }
    std::cout << std::endl;
}
