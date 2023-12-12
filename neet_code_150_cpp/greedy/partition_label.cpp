#include <iostream>
#include <vector>
#include <string>

class Solution {
    public:
        std::vector<int> partitionLabels(std::string s) {
            return std::vector{1};
        }
};

class SolutionAns {
public:
    std::vector<int> partitionLabels(std::string s) {
        int n = s.size();
        // その文字が最後に何番目にあったを記録
        std::vector<int> lastIndex(26);
        for (int i = 0; i < n; i++) {
            lastIndex[s[i] - 'a'] = i;
        }
        
        int size = 0;
        int end = 0;
        std::vector<int> result;
        // ここまで理解した
        
        for (int i = 0; i < n; i++) {
            size++;
            // constantly checking for further indices if possible
            end = std::max(end, lastIndex[s[i] - 'a']);
            if (i == end) {
                result.push_back(size);
                size = 0;
            }
        }
        
        return result;
    }
};

int main(void) {
    auto case_1 = std::string{"ababcbacadefegdehijhklij"};
    // [9,7,8]
    auto case_2 = std::string{"eccbbbbdec"};
    // [10]

    SolutionAns s_ans;

    // case_1
    auto res_1 = s_ans.partitionLabels(case_1);
    for(auto &x: res_1) {
        std::cout << x << " ";
    }
    std::cout << std::endl;
    // case_2
    auto res_2 = s_ans.partitionLabels(case_2);
    for(auto &x: res_2) {
        std::cout << x << " ";
    }
    std::cout << std::endl;
}
