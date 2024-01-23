#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>

// Rustの模範解答より
class SolutionAnsRust {
    public:
        int characterReplacement(std::string s, int k) {
            long long l = 0, max_f = 0;
            long long res = 0;
            std::unordered_map<char, long long> count;

            for(long long r = 0; r < s.size(); ++r) {
                if(count.find(s[r]) != count.end()) {
                    ++count[s[r]];
                } else {
                    count.insert({s[r], 1});
                }

                // max_f: r番目までで一番多く含まれている文字の文字数
                max_f = std::max(max_f, count[s[r]]);

                // r - l + 1 - max_f: [r,l]の部分文字列をmax_f個含まれる文字で置き換えるのに必要な回数
                // この値がkより大きければその置き換えは不可能
                // この回数がkに等しくなるまでlを増やす(部分文字列の開始点を先に進める)
                while(r - l + 1 - max_f > k) {
                    --count[s[l]];
                    ++l;
                }

                res = std::max(res, r - l + 1);
            }

            return res;
        }
};

// 模範解答
class SolutionAns {
    public:
        int characterReplacement(std::string s, int k) {
            std::vector<int> count(26);
            int maxCount = 0;

            int i = 0;
            int j = 0;

            int result = 0;

            while (j < s.size()) {
                count[s[j] - 'A']++;
                maxCount = std::max(maxCount, count[s[j] - 'A']);
                if (j - i + 1 - maxCount > k) {
                    count[s[i] - 'A']--;
                    i++;
                }
                result = std::max(result, j - i + 1);
                j++;
            }

            return result;
        }
};

int main(void) {
    auto case_1 = std::pair(std::string{"ABAB"}, 2);
    auto case_2 = std::pair(std::string{"AABABBA"}, 1);

    SolutionAnsRust s_ans_rs;
    std::cout << s_ans_rs.characterReplacement(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans_rs.characterReplacement(case_2.first, case_2.second) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.characterReplacement(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.characterReplacement(case_2.first, case_2.second) << std::endl;
}
