#include <iostream>
#include <vector>
#include <algorithm>

// triplets[i] = [a_i, b_i, c_i]
// target = [x, y, z]
// for i \ne j update triplets[j] by
// triplets[j] = [max(a_i,a_j), max(b_i,b_j), max(c_i,c_j)]

class Solution {
    public:
        bool mergeTriplet(std::vector<std::vector<int>>& triplets, std::vector<int>& target) {
            int n = triplets.size();
            for(int i = 0; i < n; ++i) {
            }

            return true;
        }
};

int main(void) {
    auto case_1_tri = std::vector{std::vector{2, 5, 3}, std::vector{1, 8, 4}, std::vector{1, 7, 5}};
    auto case_1_target = std::vector{2, 7, 5}; 
    // true
    auto case_2_tri = std::vector{std::vector{3, 4, 5}, std::vector{4, 5, 6}};
    auto case_2_target = std::vector{3, 2, 5}; 
    // false
    auto case_3_tri = std::vector{std::vector{2, 5, 3}, std::vector{2, 3, 4}, std::vector{1, 2, 5}};
    auto case_3_target = std::vector{5, 5, 5}; 
    // true
}
