#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>
#define DEBUG_PRINT

class Node {
    public:
        using MapCharNode = std::unordered_map<char, Node*>;
        MapCharNode children;
        bool isWord;

        Node() {
            for(char c = 'a'; c <= 'z'; ++c) {
                children[c] = nullptr;
            }
            isWord = false;
        }
};

// 解けなかった
class WordDictionary {
    public:
        WordDictionary() {
            root = new Node();
        }

        void addWord(std::string word) {
            auto node = root;

            for(auto& c: word) {
                if(!(node->children[c])) {
                    node->children[c] = new Node();
                }
                node = node->children[c];
            }
            node->isWord = true;
        }

        bool search(std::string word) {
            auto node = root;

            for(auto& c: word) {
#ifdef DEBUG_PRINT
                std::cout << c << " ";
#endif
                if(c == '.') {
                    continue;
                }

                if(!(node->children[c])) {
#ifdef DEBUG_PRINT
            std::cout << std::endl;
#endif
                    return false;
                }

                node = node->children[c];
            }
#ifdef DEBUG_PRINT
            std::cout << std::endl;
#endif

            return node->isWord;
        }

    private:
        Node* root;
};

// 模範解答
class NodeAns {
    public:
        NodeAns* children[26];
        bool isWord;

        NodeAns(void) {
            for(int i = 0; i < 26; ++i) {
                children[i] = nullptr;
            }
            isWord = false;
        }
};

class WordDictionaryAns {
    public:
        WordDictionaryAns(void) {
            root = new NodeAns();
        }

        void addWord(std::string word) {
            auto node = root;
            int current = 0;

            for(auto& c: word) {
                current = c - 'a';
                if(!(node->children[current])) {
                    node->children[current] = new NodeAns();
                }

                node = node->children[current];
            }

            node->isWord = true;
        }

        bool search(std::string word) {
            auto node = root;
            return searchInNode(word, 0, node);
        }

    private:
        NodeAns* root;

        bool searchInNode(std::string& word, int i, NodeAns* node) {
            if(!node) {
                return false;
            }
            if(i == word.size()) {
                return node->isWord;
            }
            if(word[i] != '.') {
                return searchInNode(word, i + 1, node->children[word[i] - 'a']);
            }
            for(int j = 0; j < 26; ++j) {
                if(searchInNode(word, i + 1, node->children[j])) {
                    return true;
                }
            }

            return false;
        }
};

int main(void) {
    using Case = std::pair<std::vector<std::string>, std::vector<std::string>>;
    // '.': すべての文字にマッチ 
    Case case_1 = {
        {"bad", "dad", "mad"}, 
        {"pad", "bad", ".ad", "b..", "b.d"}
        //false, true, true, true, true
    };

    WordDictionary w_dict;
    /*
    for(auto& v: case_1.first) {
        w_dict.addWord(v);
    }
    for(auto& v: case_1.second) {
        std::cout << w_dict.search(v) << '\n';
    }
    */

    WordDictionaryAns w_dict_ans;
    for(auto& v: case_1.first) {
        w_dict_ans.addWord(v);
    }
    for(auto& v: case_1.second) {
        std::cout << w_dict_ans.search(v) << '\n';
    }
}
