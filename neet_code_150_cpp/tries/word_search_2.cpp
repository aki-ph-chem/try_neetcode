#include <iostream>
#include <vector>
#include <string>

class Node {
    public:
        Node* children[26];
        bool isWord;

        Node(void) {
            for(int i = 0; i < 26; ++i) {
                children[i] = nullptr;
            }
            isWord = false;
        }
};

// 解けなかった
class Solution {
    public:
        std::vector<std::string> findWords(
                std::vector<std::vector<char>>& board, 
                std::vector<std::string>& words
                ) {
            auto [m,n] = std::pair(board.size(), board[0].size());
            std::vector<std::string> result;

            for(auto& w: words) {
                for(int i = 0; i < m; ++i) {
                    for(int j = 0; j < n; ++j) {
                        if(board[i][j] == w[0]) {
                            if(dfs(board, w, 0, i, j)) {
                                result.push_back(w);
                            }
                        }
                    }
                }
            }

            return result;
        }

    private:
        bool dfs(std::vector<std::vector<char>>& board,
                std::string word,
                int index,
                int i,
                int j
                ) {
            auto [m,n] = std::pair(board.size(), board[0].size());

            if(i < 0 
                    || i >= m
                    || j < 0
                    || j >= n
                    || board[i][j] != word[index]
              ) {
                return false;
            }
            if(index == (int)word.size() - 1) {
                return true;
            }

            board[i][j] = '#';
            if(dfs(board, word, index + 1, i - 1 ,j)
                    || dfs(board, word, index + 1, i + 1,j)
                    || dfs(board, word, index + 1, i ,j - 1)
                    || dfs(board, word, index + 1, i ,j + 1)
                    ) {
                return true;
            }

            board[i][j] = word[index];
            return false;
        }
};

// 模範解答
class SolutionAns {
    public:
        std::vector<std::string> findWords(std::vector<std::vector<char>>& board,
                std::vector<std::string>& words
                ) {
            for(auto& w: words) {
                insert(w);
            }

            auto [m, n] = std::pair(board.size(), board[0].size());
            auto node = root;
            std::vector<std::string> result;

            for(int i = 0; i < m; ++i) {
                for(int j = 0; j < n; ++j) {
                    search(board, i, j, m, n, node, "", result);
                }
            }

            return result;
        }

    private:
        Node* root = new Node();

        void insert(std::string word) {
            auto node = root;
            int current = 0;

            for(auto& c: word) {
                current = c - 'a';
                if(!(node->children[current])) {
                    node->children[current] = new Node();
                }
                node = node->children[current];
            }

            node->isWord = true;
        }

        void search(std::vector<std::vector<char>>& board, 
                int i,
                int j,
                int m,
                int n,
                Node* node,
                std::string word,
                std::vector<std::string>& result
                ) {
            if(i < 0 || i >= m || j < 0 || j >= n || board[i][j] == '#') {
                return;
            }

            auto c = board[i][j];

            node = node->children[c - 'a'];
            if(!node) {
                return;
            }

            word += board[i][j];
            if(node->isWord) {
                result.push_back(word);
                node->isWord = false;
            }
            
            board[i][j] = '#';

            search(board, i - 1, j, m, n, node, word, result);
            search(board, i + 1, j, m, n, node, word, result);
            search(board, i, j - 1, m, n, node, word, result);
            search(board, i, j + 1, m, n, node, word, result);

            board[i][j] = c;
        }
};

void show_result(std::vector<std::string>& result) {
    for(auto&w :result) {
        std::cout <<w << " ";
    }
    std::cout << '\n';
}

int main(void) {
    using Board = std::vector<std::vector<char>>;
    using Case = std::pair<Board, std::vector<std::string>>;

    Case case_1 = {
        {
            {'o','a','a','n'}, 
            {'e','t','a','e'},
            {'i','h','k','r'},
            {'i','f','l','v'}
        },
        {"oath","pea","eat","rain"}
    };
    // => ["eat","oath"]
    Case case_2 = {
        {
            {'a','b'},
            {'c','d'}
        },
        {"abcd"}
    };
    // => []

    /*
    Solution s_1;
    auto res_1 = s_1.findWords(case_1.first, case_1.second);
    show_result(res_1);
    */

    SolutionAns s_ans;
    auto res_1_ans = s_ans.findWords(case_1.first, case_1.second);
    show_result(res_1_ans);
}
