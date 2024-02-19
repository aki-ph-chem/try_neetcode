#include <algorithm>
#include <iostream>
#include <iterator>
#include <unordered_map>
#include <vector>
#include <string>
#include <unordered_set>

// 解けなかった
class Solution {
    public:
        std::string foreignDictionary(std::vector<std::string>& words) {
            std::unordered_set<char> set;
            std::string result;

            for(int i = 0; i <= 100; ++i) {
                for(auto& w: words) {
                    if(w.size() <= i) {
                        continue;
                    }

                    if(set.find(w[i]) == set.end()) {
                        set.insert(w[i]);
                        result.push_back(w[i]);
                    }
                }
            }

            return result;
        }
};

// 模範解答
class SolutionAns {
    private:
        std::unordered_map<char, std::unordered_set<char>> adj;
        std::unordered_map<char, bool> visited;
        std::string result;

        bool dfs(char ch) {
            if(visited.find(ch) != visited.end()) {
                return visited[ch];
            }

            visited[ch] = true;
            for(auto next: adj[ch]) {
                if(dfs(next)) {
                    return true;
                }
            }

            visited[ch] = false;
            result.push_back(ch);
            
            return false;
        }

    public:
        std::string foreignDictionary(std::vector<std::string>& words) {
            for(const auto& word: words) {
                for(auto ch: word) {
                    adj[ch];
                }
            }

            for(int i = 0; i < (int)words.size() - 1; ++i) {
                const std::string& w_1 = words[i];
                const std::string& w_2 = words[i + 1];

                auto minLen = std::min(w_1.length(), w_2.length());
                // 空文字列を返すケース
                if(w_1.length() > w_2.length() && w_1.substr(0, minLen) == w_2.substr(0, minLen)) {
                    return "";
                }

                for(int j = 0; j < minLen; ++j) {
                    if(w_1[j] != w_2[j]) {
                        adj[w_1[j]].insert(w_2[j]);
                        break;
                    }
                }

            }

            // 空文字列を返すケース
            for(auto& pair: adj) {
                if(dfs(pair.first)) {
                    return "";
                }
            }

            std::reverse(result.begin(), result.end());
            return result;
        }
};

int main(void) {
    using Case = std::vector<std::string>;

    Case case_1 = {"z", "o"};
    // => "zo"
    Case case_2 = {"hrn","hrf","er","enn","rfnn"};
    // => "hernf"
    Case case_3 = {"wrtkj","wrt"};
    // => ""

    Solution s_1;
    std::cout << s_1.foreignDictionary(case_1) << std::endl;
    std::cout << s_1.foreignDictionary(case_2) << std::endl;

    SolutionAns s_ans;
    std::cout << s_ans.foreignDictionary(case_1) << std::endl;
    std::cout << s_ans.foreignDictionary(case_2) << std::endl;
}
