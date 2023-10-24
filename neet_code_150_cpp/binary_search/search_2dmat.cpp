#include <vector>
#include <iostream>

class Solution {
    public:
        // O(log(nm))
        // AC
        bool search_matrix(const std::vector<std::vector<int>> &matrix, int target) {
            int left_y = 0, right_y = matrix.size() - 1;
            int mid_y = (left_y + right_y) / 2;
            while(left_y <= right_y) {
                if(matrix[mid_y][0] == target) {
                    return true;
                }

                if(matrix[mid_y][0] < target) {
                    left_y = mid_y + 1;
                } else {
                    right_y = mid_y - 1;
                }

                mid_y = (left_y + right_y) / 2;
            }

            int left_x = 0, right_x = matrix[mid_y].size() - 1;
            int mid_x = (left_x + right_x) / 2;
            while(left_x <= right_x) {
                if(matrix[mid_y][mid_x] == target) {
                    return true;
                }

                if(matrix[mid_y][mid_x] < target) {
                    left_x = mid_x + 1;
                } else {
                    right_x = mid_x - 1;
                }

                mid_x = (left_x + right_x) / 2;
            }

            return false;
        }
};

int main(void) {
    Solution s_1;

    auto case_1_matrix = std::vector{std::vector{1, 3, 5, 7}, std::vector{10, 11, 16, 20}, std::vector{20, 30, 34, 60}};
    auto case_1_target = 3;

    auto case_2_matrix = std::vector{std::vector{1, 3, 5, 7}, std::vector{10, 11, 16, 20}, std::vector{20, 30, 34, 60}};
    auto case_2_target = 13;

    std::cout << s_1.search_matrix(case_1_matrix, case_1_target) << std::endl;
    std::cout << s_1.search_matrix(case_2_matrix, case_2_target) << std::endl;
}
