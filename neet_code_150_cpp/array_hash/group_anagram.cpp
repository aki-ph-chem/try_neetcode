#include <cstddef>
#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>

//#define CXX_20

class Solution {
    public:
        std::vector<std::vector<std::string>> group_anagrams(const std::vector<std::string>& strs) {
            std::unordered_map<std::string, std::vector<std::string>> map;
            for(const auto &s: strs) {
                // a ~ zのバケット
                std::string key(26, 0);
                for(const auto &c: s) {
                    key[c - 'a'] += 1;
                }

#ifdef CXX_20
                if(map.contains(key)) { // need C++ 20
#else 
                    if(map.find(key) != map.end()) {
#endif
                    map[key].push_back(s);
                } else {
                    map.insert(std::make_pair(key, std::vector{s}));
                }
            }
            std::vector<std::vector<std::string>> result;
            for(const auto &v: map) {
                result.push_back(v.second);
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::vector<std::vector<std::string>> groupAnagrams(const std::vector<std::string>& strs) {
            std::unordered_map<std::string, std::vector<std::string>> m;
            for (std::size_t i = 0; i < strs.size(); i++) {
                std::string key = getKey(strs[i]);
                m[key].push_back(strs[i]);
            }

            std::vector<std::vector<std::string>> result;
            for (auto it = m.begin(); it != m.end(); it++) {
                result.push_back(it->second);
            }
            return result;
        }
    private:
        std::string getKey(const std::string& str) {
            // a~z 26字分のバケット
            std::vector<int> count(26);
            for (std::size_t j = 0; j < str.size(); j++) {
                count[str[j] - 'a']++;
            }

            // std::vector<int> -> std::string
            std::string key = "";
            for (std::size_t i = 0; i < count.size(); i++) {
                key.append(std::to_string(count[i]) + '#');
            }
            return key;
        }
};

int main(void) {
    auto case_1 = std::vector<std::string>{"eat","tea","tan","ate","nat","bat"};
    auto case_2 = std::vector<std::string>{""};
    auto case_3 = std::vector<std::string>{"a"};

    Solution s_1;
    for(const auto &v: s_1.group_anagrams(case_1)) {
        for(const auto &w: v) {
            std::cout << w << " ";
        }
        std::cout << std::endl;
    }
    for(const auto &v: s_1.group_anagrams(case_2)) {
        for(const auto &w: v) {
            std::cout << w << " ";
        }
        std::cout << std::endl;
    }
    for(const auto &v: s_1.group_anagrams(case_3)) {
        for(const auto &w: v) {
            std::cout << w << " ";
        }
        std::cout << std::endl;
    }

    SolutionAns s_2;
    for(const auto &v: s_2.groupAnagrams(case_1)) {
        for(const auto &w: v) {
            std::cout << w << " ";
        }
        std::cout << std::endl;
    }
    for(const auto &v: s_2.groupAnagrams(case_2)) {
        for(const auto &w: v) {
            std::cout << w << " ";
        }
        std::cout << std::endl;
    }
    for(const auto &v: s_2.groupAnagrams(case_3)) {
        for(const auto &w: v) {
            std::cout << w << " ";
        }
        std::cout << std::endl;
    }
}
