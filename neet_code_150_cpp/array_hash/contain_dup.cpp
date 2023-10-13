#include <cwchar>
#include <iostream>
#include <vector>
#include <set>

// Rustで同じように実装した場合と比べてひと桁遅い
class Solution {
    public:
        bool contains_duplicate(const std::vector<int>& nums){
            std::set<int> set; 
            std::size_t num_set_size{0};
            for(const auto &v: nums) {
                ++num_set_size;
                set.insert(v);
                if(set.size() != num_set_size) {
                    return true;
                }
            }
            return false;
        }

        // need C++ 20
        bool contains_duplicate_2(const std::vector<int>& nums) {
            std::set<int> set; 
            for(const auto &v: nums) {
                if(set.contains(v)) {
                    return true;
                }
                set.insert(v);
            }
            return false;
        }

        // 模範解答とほぼ同じ(模範解答は普通のforで書いている)
        bool contains_duplicate_3(const std::vector<int>& nums) {
            std::set<int> set; 
            for(const auto &v: nums) {
                if(set.find(v) != set.end()) {
                    return true;
                }
                set.insert(v);
            }
            return false;
        }

};

int main(void) {
    Solution s;

    auto case_1 = std::vector{1, 2, 1, 3};
    auto case_2 = std::vector{1, 2, 3, 4};
    auto case_3 = std::vector{1, 1, 1, 3, 3, 4, 3, 2, 4, 2};

    std::cout << "contains_duplicate()" << std::endl;
    std::cout << "case_1: " << std::endl;
    if(s.contains_duplicate(case_1)){
        std::cout << "duplicate" << std::endl;
    };
    std::cout << "case_2: " << std::endl;
    if(s.contains_duplicate(case_2)){
        std::cout << "duplicate" << std::endl;
    };
    std::cout << "case_3: " << std::endl;
    if(s.contains_duplicate(case_3)){
        std::cout << "duplicate" << std::endl;
    };

    std::cout << "contains_duplicate_2()" << std::endl;
    std::cout << "case_1: " << std::endl;
    if(s.contains_duplicate_2(case_1)){
        std::cout << "duplicate" << std::endl;
    };
    std::cout << "case_2: " << std::endl;
    if(s.contains_duplicate_2(case_2)){
        std::cout << "duplicate" << std::endl;
    };
    std::cout << "case_3: " << std::endl;
    if(s.contains_duplicate_2(case_3)){
        std::cout << "duplicate" << std::endl;
    };

    std::cout << "contains_duplicate_3()" << std::endl;
    std::cout << "case_1: " << std::endl;
    if(s.contains_duplicate_3(case_1)){
        std::cout << "duplicate" << std::endl;
    };
    std::cout << "case_2: " << std::endl;
    if(s.contains_duplicate_3(case_2)){
        std::cout << "duplicate" << std::endl;
    };
    std::cout << "case_3: " << std::endl;
    if(s.contains_duplicate_3(case_3)){
        std::cout << "duplicate" << std::endl;
    };
}
