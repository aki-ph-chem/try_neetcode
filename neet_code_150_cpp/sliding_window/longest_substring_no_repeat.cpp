#include <iostream>
#include <vector>
#include <unordered_set>
#include <deque>
#include <algorithm>

class Solution {
    public:
        // AC
        // O(N^2)で遅い
        int lengthOfLongestSubstringSq(std::string s) {
            if(!s.size()) {
                return 0;
            }

            std::unordered_set<char> set;
            std::vector<int> len_list;
            int len_tmp = 1;

            for(int i = 0; i < s.size(); ++i) {
                set.insert(s[i]);
                for(int j = i + 1; j < s.size(); ++j) {
                    if(set.find(s[j]) == set.end()) {
                        set.insert(s[j]);
                        ++len_tmp;
                    } else {
                        break;
                    }
                }

                len_list.push_back(len_tmp);
                set.clear();
                len_tmp = 1;
            }

            return *std::max_element(len_list.begin(), len_list.end());
        }

};

// Rustの模範解答より
class SolutionAnsRust {
    public:
        // AC
        // O(N^2)
        int lengthOfLongestSubstring(std::string s) {
            std::deque<char> set;
            int result = 0;

            auto is_contain = [&](char c) {
                for(auto& c_dq: set) {
                    if(c == c_dq) {
                        return true;
                    }
                }

                return false;
            };

            for(auto& c : s) {
                while(is_contain(c)) {
                    set.pop_front();
                }

                set.push_back(c);
                result = std::max(result, (int)set.size());
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        int lengthOfLongestSubstring(std::string& s) {
            std::unordered_set<char> chars;
            int maxSize = 0;
            int i = 0, j = 0;
            while (j < s.size()){
                while (chars.find(s[j]) != chars.end()){
                    chars.erase(s[i]);
                    ++i;
                }
                maxSize = std::max(maxSize, j - i + 1);
                chars.insert(s[j]);
                ++j;
            }
            return maxSize;
        }
};

int main(void) {
    std::string case_1 = "abcabcbb";
    std::string case_2 = "bbbbb";
    std::string case_3 = "pwwkew";
    std::string case_4 = "xyz";
    std::string case_5 = "au";
    std::string case_6 = "a";
    std::string case_7 = "dvdf";

    Solution s_1;

    std::cout << "case_1 " << s_1.lengthOfLongestSubstringSq(case_1) << std::endl;
    std::cout << "case_2 " << s_1.lengthOfLongestSubstringSq(case_2) << std::endl;
    std::cout << "case_3 " << s_1.lengthOfLongestSubstringSq(case_3) << std::endl;
    std::cout << "case_4 " << s_1.lengthOfLongestSubstringSq(case_4) << std::endl;
    std::cout << "case_5 " << s_1.lengthOfLongestSubstringSq(case_5) << std::endl;
    std::cout << "case_6 " << s_1.lengthOfLongestSubstringSq(case_6) << std::endl;
    std::cout << "case_7 " << s_1.lengthOfLongestSubstringSq(case_7) << std::endl;

    SolutionAnsRust s_ans_rs;

    std::cout << "case_1 " << s_ans_rs.lengthOfLongestSubstring(case_1) << std::endl;
    std::cout << "case_2 " << s_ans_rs.lengthOfLongestSubstring(case_2) << std::endl;
    std::cout << "case_3 " << s_ans_rs.lengthOfLongestSubstring(case_3) << std::endl;
    std::cout << "case_4 " << s_ans_rs.lengthOfLongestSubstring(case_4) << std::endl;
    std::cout << "case_5 " << s_ans_rs.lengthOfLongestSubstring(case_5) << std::endl;
    std::cout << "case_6 " << s_ans_rs.lengthOfLongestSubstring(case_6) << std::endl;
    std::cout << "case_7 " << s_ans_rs.lengthOfLongestSubstring(case_7) << std::endl;

    SolutionAns s_ans;
    std::cout << "case_1 " << s_ans.lengthOfLongestSubstring(case_1) << std::endl;
    std::cout << "case_2 " << s_ans.lengthOfLongestSubstring(case_2) << std::endl;
    std::cout << "case_3 " << s_ans.lengthOfLongestSubstring(case_3) << std::endl;
    std::cout << "case_4 " << s_ans.lengthOfLongestSubstring(case_4) << std::endl;
    std::cout << "case_5 " << s_ans.lengthOfLongestSubstring(case_5) << std::endl;
    std::cout << "case_6 " << s_ans.lengthOfLongestSubstring(case_6) << std::endl;
    std::cout << "case_7 " << s_ans.lengthOfLongestSubstring(case_7) << std::endl;
}
