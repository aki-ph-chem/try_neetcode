#include <iostream>
#include <climits>
#include <string>
#include <unordered_map>
//#define DEBUG_P

// 模範解答
class SolutionAns {
public:
    std::string minWindow(std::string s, std::string t) {
        // count of char in t
        std::unordered_map<char, int> m;
        for (int i = 0; i < t.size(); i++) {
            m[t[i]]++;
        }
        
        int i = 0;
        int j = 0;
        
        // # of chars in t that must be in s
        int counter = t.size();
        
        int minStart = 0;
        int minLength = INT_MAX;
        
        while (j < s.size()) {
            // if char in s exists in t, decrease
            if (m[s[j]] > 0) {
                counter--;
            }
            // if char doesn't exist in t, will be -'ve
            m[s[j]]--;
            // move j to find valid window
            j++;
            
            // when window found, move i to find smaller
            while (counter == 0) {
                if (j - i < minLength) {
                    minStart = i;
                    minLength = j - i;
                }
                
                m[s[i]]++;
                // when char exists in t, increase
                if (m[s[i]] > 0) {
                    counter++;
                }
                i++;
            }
        }
#ifdef DEBUG_P
        std::cout << minStart << "," << minLength << std::endl; 
#endif
        
        if (minLength != INT_MAX) {
            return s.substr(minStart, minLength);
        }
        return "";
    }
};

int main(void) {
    SolutionAns s_ans;

    auto case_1 = std::pair(std::string{"ADOBECODEBANC"}, std::string{"ABC"});
    auto case_2 = std::pair(std::string{"a"}, std::string{"a"});
    auto case_3 = std::pair(std::string{"a"}, std::string{"aa"});

    std::cout << s_ans.minWindow(case_1.first, case_1.second) << std::endl;
    std::cout << s_ans.minWindow(case_2.first, case_2.second) << std::endl;
    std::cout << s_ans.minWindow(case_3.first, case_3.second) << std::endl;
}
