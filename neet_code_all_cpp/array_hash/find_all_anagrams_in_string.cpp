#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>

class Solution {
    public:
        // AC
        std::vector<int> findAnagrams(std::string s, std::string p) {
            if(s.length() < p.length()) {
                return std::vector<int>{};
            }

            std::unordered_map<char, int> map, map_tmp;
            std::vector<int> result;
            for(auto i = 0; i < p.length(); ++i) {
                ++map[p[i]];
                ++map_tmp[s[i]];
            }
            if(map_tmp == map) {
                result.push_back(0);
            }

            int left = 0;
            for(auto i = p.length(); i < s.length(); ++i) {
                ++map_tmp[s[i]];
                if(map_tmp[s[left]] == 1) {
                    map_tmp.erase(s[left]);
                } else {
                    --map_tmp[s[left]];
                }

                if(map_tmp == map) {
                    result.push_back(left + 1);
                }

                ++left;
            }

            return result;
        }
};

// 模範解答(大幅に書き換えてはいるが...)
class SolutionAns {
    public:
        std::vector<int> findAnagrams(std::string s, std::string p) {
            auto gen_map = [](std::string p) {
                std::unordered_map<char, int> map;
                for(auto& c: p) {
                    ++map[c];
                }

                return map;
            };

            std::vector<int> result;
            int p_f = 0, p_s = p.length();
            int len_s = s.length(), len_p = p.length();
            auto map_p = gen_map(p); 
            auto map_s = gen_map(s.substr(0, p.length()));

            for(p_f = 0; p_f < len_s - len_p + 1; ++p_f) {
                if(map_s == map_p) {
                    result.push_back(p_f);
                }

                ++map_s[s[p_s]];
                ++p_s;
                --map_s[s[p_f]];
                if(map_s[s[p_f]] == 0) {
                    map_s.erase(s[p_f]);
                }
                std::cout << "p_s: " << p_s << " s[p_s]: " << s[p_s] << std::endl;
            }

            return result;
        }
};

void show_result(std::vector<int>& result) {
    for(auto&v: result) {
        std::cout << v << " ";
    }
    std::cout << std::endl;
}

int main(void) {
    using Case = std::pair<std::string, std::string>;
    Case case_1 = {"cbaebabacd", "abc"};
    // => [0, 6]
    Case case_2 = {"abab", "ab"};
    // => [0, 1, 2]

    Solution s_1;

    auto res_1 = s_1.findAnagrams(case_1.first, case_1.second);
    show_result(res_1);
    auto res_2 = s_1.findAnagrams(case_2.first, case_2.second);
    show_result(res_2);

    SolutionAns s_ans;

    auto res_1_ans = s_ans.findAnagrams(case_1.first, case_1.second);
    show_result(res_1_ans);
    auto res_2_ans = s_ans.findAnagrams(case_2.first, case_2.second);
    show_result(res_2_ans);
}
