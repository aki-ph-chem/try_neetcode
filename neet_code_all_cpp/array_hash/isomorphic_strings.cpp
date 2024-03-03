#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

class Solution {
    public:
        // AC
        bool isIsomorphic(std::string s, std::string t) {
            std::unordered_map<char, std::vector<int>> map_s;
            std::unordered_map<char, std::vector<int>> map_t;

            for(int i = 0; i < s.length(); ++i) {
                if(map_s.find(s[i]) != map_s.end()) {
                    map_s[s[i]].push_back(i);
                } else {
                    map_s.insert({s[i], std::vector{i}});
                }

                if(map_t.find(t[i]) != map_t.end()) {
                    map_t[t[i]].push_back(i);
                } else {
                    map_t.insert({t[i], std::vector{i}});
                }
            }

            for(int i = 0; i < s.length(); ++i) {
                if(map_s[s[i]] != map_t[t[i]]) {
                    return false;
                }
            }

            return true;
        }
};

// 模範解答
class SolutionAns {
    public:
        bool isIsomorphic(std::string s, std::string t) {
            std::unordered_map<char, std::vector<int>> m1, m2;

            for(int i = 0; i < s.length(); ++i) {
                m1[s[i]].push_back(i);
                m2[t[i]].push_back(i);

                if(m1[s[i]] != m2[t[i]]) {
                    return false;
                }
            }

            return true;
        }
};

int main(void) {
    using Case = std::pair<std::string, std::string>;

    Case case_1 = {"egg", "add"};
    Case case_2 = {"foo", "bar"};
    Case case_3 = {"paper", "title"};

    Solution s_1;
    std::cout << s_1.isIsomorphic(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.isIsomorphic(case_2.first, case_2.second) << std::endl;
    std::cout << s_1.isIsomorphic(case_3.first, case_3.second) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.isIsomorphic(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.isIsomorphic(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.isIsomorphic(case_3.first, case_3.second) << std::endl;
}
