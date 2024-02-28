#include <iostream>
#include <algorithm>
#include <array>
#include <string>
#include <map>
#include <unordered_map>

// どれもRustより遅い
class Solution {
    public:
        //bool is_anagram(std::string& s, std::string& t) {
        bool is_anagram(std::string&& s, std::string&& t) {
            if(s.length() != t.length()) {
                return false;
            } 

            std::sort(s.begin(), s.end());
            std::sort(t.begin(), t.end());

            if(s == t) {
                return true;
            }

            return false;
        }

        //bool is_anagram_2(const std::string& s, const std::string& t) {
        bool is_anagram_2(const std::string&& s, const std::string&& t) {
            if(s.length() != t.length()) {
                return false;
            } 

            std::map<char,std::size_t> map;
            for(const auto &c: s) {
                if(map.find(c) != map.end()) {
                    map[c] += 1;
                } else {
                    map.insert(std::make_pair(c, 1));
                }
            }
            for(const auto &c: t) {
                if(map.find(c) != map.end()) {
                    map[c] -= 1;
                } else {
                    map.insert(std::make_pair(c, 1));
                }
            }

            for(const auto v: map) {
                if(v.second != 0) {
                    return false;
                }
            }

            return true;
        }

        //bool is_anagram_3(const std::string& s, const std::string& t) {
        bool is_anagram_3(const std::string&& s, const std::string&& t) {
            if(s.length() != t.length()) {
                return false;
            } 

            std::map<char,std::size_t> map;
            for(std::size_t i = 0; i < s.length(); ++i) {
                if(map.find(s[i]) == map.end()) {
                    map.insert(std::make_pair(s[i], 0));
                }
                map[s[i]] += 1;

                if(map.find(t[i]) == map.end()) {
                    map.insert(std::make_pair(t[i], 0));
                }
                map[t[i]] -= 1;
            }

            for(const auto v: map) {
                if(v.second != 0) {
                    return false;
                }
            }

            return true;
        }

        // AC
        bool is_anagram_4(const std::string&& s, const std::string&& t) {
            if(t.size() != s.size()) return false;
            std::unordered_map<char, int> map;

            for(int i = 0; i < s.size(); ++i) {
                ++map[s[i]];
                --map[t[i]];
            }

            for(auto& [idx, v]: map){
                if(v != 0) {
                    return false;
                }
            }

            return true;
        }
};

// 模範解答
class SolutionAns {
    public:
        bool isAnagram(const std::string&& s, const std::string&& t) {
            if(s.size() != t.size()) return false;

            std::unordered_map<char,int> smap;
            std::unordered_map<char,int> tmap;

            for(std::size_t i = 0; i < s.size(); i++){
                smap[s[i]]++;
                tmap[t[i]]++;
            }

            for(std::size_t i = 0; i < smap.size(); i++){
                if(smap[i] != tmap[i]) return false;
            }
            return true;
        }
};

int main(void) {

    Solution s_1;

    auto case_1 = std::array{"anagram", "nagaram"};
    auto case_2 = std::array{"rat", "car"};

    std::cout << "is_anagram()" << std::endl;
    std::cout << "case_1" << std::endl;
    if(s_1.is_anagram(case_1[0],case_1[1])) {
        std::cout << "valid anagram" << std::endl;
    }
    std::cout << "case_2" << std::endl;
    if(s_1.is_anagram(case_2[0],case_2[1])) {
        std::cout << "valid anagram" << std::endl;
    }

    std::cout << "is_anagram_2()" << std::endl;
    std::cout << "case_1" << std::endl;
    if(s_1.is_anagram_2(case_1[0],case_1[1])) {
        std::cout << "valid anagram" << std::endl;
    }
    std::cout << "case_2" << std::endl;
    if(s_1.is_anagram_2(case_2[0],case_2[1])) {
        std::cout << "valid anagram" << std::endl;
    }

    std::cout << "is_anagram_3()" << std::endl;
    std::cout << "case_1" << std::endl;
    if(s_1.is_anagram_3(case_1[0],case_1[1])) {
        std::cout << "valid anagram" << std::endl;
    }
    std::cout << "case_2" << std::endl;
    if(s_1.is_anagram_3(case_2[0],case_2[1])) {
        std::cout << "valid anagram" << std::endl;
    }

    std::cout << "is_anagram_4()" << std::endl;
    std::cout << "case_1" << std::endl;
    if(s_1.is_anagram_4(case_1[0],case_1[1])) {
        std::cout << "valid anagram" << std::endl;
    }
    std::cout << "case_2" << std::endl;
    if(s_1.is_anagram_4(case_2[0],case_2[1])) {
        std::cout << "valid anagram" << std::endl;
    }

    SolutionAns s_2;
    std::cout << "isAnagram()" << std::endl;
    std::cout << "case_1" << std::endl;
    if(s_2.isAnagram(case_1[0],case_1[1])) {
        std::cout << "valid anagram" << std::endl;
    }
    std::cout << "case_2" << std::endl;
    if(s_2.isAnagram(case_2[0],case_2[1])) {
        std::cout << "valid anagram" << std::endl;
    }
}
