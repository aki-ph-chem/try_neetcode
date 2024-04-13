#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

class Solution {
    public:
        // AC
        std::vector<std::string> findRepeatedDnaSequences(std::string s) {
            if(s.size() < 10) {
                return std::vector<std::string>{};
            }

            auto n = (int) s.size();
            std::unordered_map<std::string, int> map;
            for(int i = 0; i <= n - 10; ++i) {
                ++map[s.substr(i, 10)];
            }

            std::vector<std::string> result;
            for(auto& [sub_str, n]: map) {
                if(n > 1) {
                    result.push_back(sub_str);
                }
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::vector<std::string> findRepeatedDnaSequences(std::string s) {
            auto n = (int)s.size(); 
            if(n <= 10) {
                return {};
            }

            std::vector<std::string> result;
            std::unordered_map<std::string, int> map;

            for(int i = 0; i <= n - 10; ++i) {
                auto sub_str = s.substr(i, 10);
                ++map[sub_str];
                if(map[sub_str] == 2) {
                    result.push_back(sub_str);
                }
            }

            return result;
        }
};

void show_result(std::vector<std::string>& result) {
    for(auto& v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    std::string case_1 = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT";
    // => ["AAAAACCCCC", "CCCCCAAAAA"]
    std::string case_2 = "AAAAAAAAAAAAA";
    // => ["AAAAAAAAAAAAA"]
    std::string case_3 = "AAAAAAAAAAA";
    // => ["AAAAAAAAAA"]

    Solution s_1;
    auto res_1 = s_1.findRepeatedDnaSequences(case_1);
    auto res_2 = s_1.findRepeatedDnaSequences(case_2);
    auto res_3 = s_1.findRepeatedDnaSequences(case_3);

    show_result(res_1);
    show_result(res_2);
    show_result(res_3);

    SolutionAns s_ans;
    auto res_1_ans = s_ans.findRepeatedDnaSequences(case_1);
    auto res_2_ans = s_ans.findRepeatedDnaSequences(case_2);
    auto res_3_ans = s_ans.findRepeatedDnaSequences(case_3);

    show_result(res_1_ans);
    show_result(res_2_ans);
    show_result(res_3_ans);
}
