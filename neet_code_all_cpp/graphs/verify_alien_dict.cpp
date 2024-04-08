#include <iostream>
#include <vector>
#include <string>
#include <unordered_map>

// Rustの模範解答より
class SolutionAnsRust {
    public:
        // AC
        bool isAlienSorted(std::vector<std::string>& words, std::string order) {
            auto n = (int)words.size();
            std::unordered_map<char, int> map;

            for(int i = 0; i < (int)order.size(); ++i) {
                map.insert({order[i], i});
            }

            for(int i = 1; i < n; ++i) {
                auto s_1 = words[i - 1];
                auto s_2 = words[i];
                auto is_equal = true;

                for(int j = 0; j < (int)std::min(s_1.size(), s_2.size()) ; ++j) {
                    if(map[s_1[j]] > map[s_2[j]]) {
                        return false;
                    }

                    if(map[s_1[j]] < map[s_2[j]]) {
                        is_equal = false;
                        break;
                    }
                }
                if(is_equal && s_1.size() > s_2.size()) {
                    return false;
                }
            }

            return true;
        }
};

int main(void) {
    using Case = std::pair<std::vector<std::string>, std::string>;
    Case case_1 = {
        {"hello", "leetcode"},
        "hlabcdefgijkmnopqrstuvwxyz",
    };
    // => true
    Case case_2 = {
        {"word", "world", "row"},
        "worldabcefghijkmnpqstuvxyz",
    };
    // => false
    Case case_3 = {
        {"apple", "app"},
        "abcdefghijklmnopqrstuvwxyz",
    };
    // => false

    SolutionAnsRust s_ans_rs;

    std::cout << s_ans_rs.isAlienSorted(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_rs.isAlienSorted(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans_rs.isAlienSorted(case_3.first, case_3.second) << std::endl;
}
