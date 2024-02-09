#include <iostream>
#include <string>
#include <tuple>
#include <vector>
#include <unordered_set>
#include <queue>

// 模範解答
class SolutionAns {
    public:
        int ladderLenght(std::string beginWord, std::string endWord, std::vector<std::string>& wordList) {
            std::unordered_set<std::string> dict;
            for(auto s: wordList) {
                dict.insert(s);
            }

            std::queue<std::string> q;
            q.push(beginWord);

            int result = 1;

            while(!q.empty()) {
                auto count = q.size();

                for(int i = 0; i < count; ++i) {
                    auto word = q.front();
                    q.pop();

                    if(word == endWord) {
                        return result;
                    }
                    dict.erase(word);

                    for(int j = 0; j < word.size(); ++j) {
                        auto c = word[j];

                        for(int k = 0; k < 26; ++k) {
                            word[j] = k + 'a'; 
                            if(dict.find(word) != dict.end()) {
                                q.push(word);
                                dict.erase(word);
                            }

                            word[j] = c;
                        }
                    }
                }

                ++result;
            }

            return 0;
        }
};
using Case = std::tuple<std::string, std::string, std::vector<std::string>>;
int main(void) {
    Case case_1 = {
        "hit", 
        "cog", 
        {
            "hot", 
            "dot", 
            "dog", 
            "lot", 
            "log", 
            "cog"
        }
    };
    Case case_2 = {
        "hit", 
        "cog", 
        {
            "hot", 
            "dot", 
            "dog", 
            "lot", 
            "log", 
        }
    };

    SolutionAns s_ans;

    std::cout << s_ans.ladderLenght(std::get<0>(case_1), std::get<1>(case_1), std::get<2>(case_1)) << std::endl;
    std::cout << s_ans.ladderLenght(std::get<0>(case_2), std::get<1>(case_2), std::get<2>(case_2)) << std::endl;
}
