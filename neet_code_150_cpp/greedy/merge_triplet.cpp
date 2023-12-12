#include <iostream>
#include <vector>
#include <algorithm>
#include <unordered_set>

// triplets[i] = [a_i, b_i, c_i]
// target = [x, y, z]
// for i \ne j update triplets[j] by
// triplets[j] = [max(a_i,a_j), max(b_i,b_j), max(c_i,c_j)]

class Solution {
    public:
        bool mergeTriplet(std::vector<std::vector<int>>& triplets, std::vector<int>& target) {
            int n = triplets.size();
            // マージするペアを全列挙 => bit全探索O(N2^N)
            for(int bit = 0; bit < (1 << n); ++bit) {
                for(int i = 1; i < n; ++i) {
                    auto trp = triplets;
                    if(bit & (i << i)) {
                        trp[i][0] = std::max(trp[i - 1][0], trp[i][0]);
                        trp[i][1] = std::max(trp[i - 1][1], trp[i][1]);
                        trp[i][2] = std::max(trp[i - 1][2], trp[i][2]);
                    }
                }
            }

            return false;
        }
};

class SolutionAns {
    public:
        bool mergeTriplets(std::vector<std::vector<int>>& triplets, std::vector<int>& target) {
            std::unordered_set<int> s;

            // taragetより大きければければmergeしてもtargetの値にならないので除外
            for (int i = 0; i < triplets.size(); i++) {
                if (triplets[i][0] > target[0] || triplets[i][1] > target[1] || triplets[i][2] > target[2]) {
                    continue;
                }

                // そうでなくかつtargetと等しいならばsに挿入
                for (int j = 0; j < 3; j++) {
                    if (triplets[i][j] == target[j]) {
                        s.insert(j);
                    }
                }
            }

            // sに３個入っていればok
            return s.size() == 3;
        }
};

int main(void) {
    auto case_1_tri = std::vector{std::vector{2, 5, 3}, std::vector{1, 8, 4}, std::vector{1, 7, 5}};
    auto case_1_target = std::vector{2, 7, 5}; 
    // true
    auto case_2_tri = std::vector{std::vector{3, 4, 5}, std::vector{4, 5, 6}};
    auto case_2_target = std::vector{3, 2, 5}; 
    // false
    auto case_3_tri = std::vector{std::vector{2, 5, 3}, std::vector{2, 3, 4}, std::vector{1, 2, 5}, std::vector{5, 2, 3}};
    auto case_3_target = std::vector{5, 5, 5}; 
    // true

    /*
    Solution s_1;
    std::cout << s_1.mergeTriplet(case_1_tri, case_1_target) << std::endl;
    std::cout << s_1.mergeTriplet(case_2_tri, case_2_target) << std::endl;
    std::cout << s_1.mergeTriplet(case_3_tri, case_3_target) << std::endl;
    */
    SolutionAns s_ans;
    std::cout << s_ans.mergeTriplets(case_1_tri, case_1_target) << std::endl;
    std::cout << s_ans.mergeTriplets(case_2_tri, case_2_target) << std::endl;
    std::cout << s_ans.mergeTriplets(case_3_tri, case_3_target) << std::endl;
}
