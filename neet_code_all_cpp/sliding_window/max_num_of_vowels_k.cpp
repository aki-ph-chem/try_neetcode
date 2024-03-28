#include <climits>
#include <iostream>
#include <string>
#include <unordered_set>

class Solution {
    public:
        // AC
        int maxVowels(std::string s, int k) {
            std::unordered_set<char> vowel_set = {'a', 'e', 'i', 'o', 'u'};
            int left = 0;
            int total = 0;
            int result = 0;

            for(int i = 0; i < k; ++i) {
                if(vowel_set.find(s[i]) != vowel_set.end()) {
                    ++total;
                }
                result = total;
            }

            for(int i = k; i < s.length(); ++i) {
                if(vowel_set.find(s[i]) != vowel_set.end()) {
                    ++total;
                }
                if(vowel_set.find(s[left]) != vowel_set.end()) {
                    --total;
                }

                ++left;
                result = std::max(result, total);
            }

            return result;
        }

        // AC
        int maxVowels2(std::string s, int k) {
            auto is_vowels = [](char& c) {
                if(c == 'a'
                        || c == 'e'
                        || c == 'i' 
                        || c == 'o'
                        || c == 'u' 
                  ){
                    return true;
                }

                return false;
            };

            int left = 0;
            int total = 0;
            int result = 0;

            for(int i = 0; i < k; ++i) {
                if(is_vowels(s[i])) {
                    ++total;
                }
                result = total;
            }

            for(int i = k; i < s.length(); ++i) {
                if(is_vowels(s[i])) {
                    ++total;
                }
                if(is_vowels(s[left])) {
                    --total;
                }

                ++left;
                result = std::max(result, total);
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    public:
        bool isVowel(char ch) {
            if(ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u') {
                return true;
            }

            return false;
        }

        int maxVowels(std::string s, int k) {
            int part_1 = 0, part_2 = 0;
            int count = 0, max_i = INT_MIN;

            while(part_2 < s.size()) {
                if(part_2 - part_1 == k) {

                    max_i = std::max(count, max_i);
                    if(max_i == k) {
                        return count;
                    }

                    if(isVowel(s[part_1++])) {
                        --count;
                    }
                    if(isVowel(s[part_2++])) {
                        ++count;
                    }

                } else {
                    if(isVowel(s[part_2])) {
                        ++count;
                    }

                    ++part_2;
                }
            }

            max_i = std::max(count, max_i);
            return max_i;
        }
};

int main(void) {
    using Case = std::pair<std::string, int>;
    Case case_1 = {"abciiidef", 3};
    // => 3
    Case case_2 = {"aeiou", 2};
    // => 2
    Case case_3 = {"leetcode", 3};
    // => 2
    Case case_4 = {"tryhard", 4};
    // => 1

    Solution s_1;

    std::cout << s_1.maxVowels(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.maxVowels(case_2.first, case_2.second) << std::endl;
    std::cout << s_1.maxVowels(case_3.first, case_3.second) << std::endl;
    std::cout << s_1.maxVowels(case_4.first, case_4.second) << std::endl;

    std::cout << s_1.maxVowels2(case_1.first, case_1.second) << std::endl;
    std::cout << s_1.maxVowels2(case_2.first, case_2.second) << std::endl;
    std::cout << s_1.maxVowels2(case_3.first, case_3.second) << std::endl;
    std::cout << s_1.maxVowels2(case_4.first, case_4.second) << std::endl;

    SolutionAns s_ans;

    std::cout << s_ans.maxVowels(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.maxVowels(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.maxVowels(case_3.first, case_3.second) << std::endl;
    std::cout << s_ans.maxVowels(case_4.first, case_4.second) << std::endl;
}
