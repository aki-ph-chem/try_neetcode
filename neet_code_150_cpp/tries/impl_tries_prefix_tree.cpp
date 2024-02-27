#include <iostream>
#include <string>
#include <unordered_map>

// 模範解答
class TrieNode {
    public:
        TrieNode* children[26];
        bool isWord;

        TrieNode() {
            for(int i = 0; i < 26; ++i) {
                children[i] = nullptr;
            }
            isWord = false;
        }
};

class TrieAns {
    public:
        TrieAns() {
            root = new TrieNode();
        }

        void insert(std::string word) {
            TrieNode* node = root;
            int curr = 0;

            for(int i = 0; i < (int)word.size(); ++i) {
                curr = word[i] - 'a';
                if(!(node->children[curr])) {
                    node->children[curr] = new TrieNode();
                }
                node = node->children[curr];
            }

            node->isWord = true;
        }

        bool search(std::string word) {
            TrieNode* node = root;
            int curr = 0;

            for(int i = 0; i < (int)word.size(); ++i){
                curr = word[i] - 'a';
                if(!(node->children[curr])) {
                    return false;
                }
                node = node->children[curr];
            }

            return node->isWord;
        }

        bool startsWith(std::string prefix) {
            TrieNode* node = root;
            int curr = 0;

            for(int i = 0; i < (int)prefix.size(); ++i) {
                curr = prefix[i] - 'a';
                if(!(node->children[curr])) {
                    return false;
                }
                node = node->children[curr];
            }

            return true;
        }

    private:
        TrieNode* root;
};

// AC
// searchを再帰で書く(dfsを意識)
class TrieAnsRec {
    public:
        TrieAnsRec() {
            root = new TrieNode();
        }

        void insert(std::string word) {
            TrieNode* node = root;
            int curr = 0;

            for(int i = 0; i < (int)word.size(); ++i) {
                curr = word[i] - 'a';
                if(!(node->children[curr])) {
                    node->children[curr] = new TrieNode();
                }
                node = node->children[curr];
            }

            node->isWord = true;
        }

        bool search(std::string word) {
            return searchRec(word);
        }

        bool searchRec(std::string word) {
            auto node = root;
            return dfs(word, 0, node);
        }

        bool dfs(std::string& word, int i, TrieNode* node) {
            if(!node) {
                return false;
            }
            if(i == word.size()) {
                return node->isWord;
            }

            return dfs(word, i + 1, node->children[word[i] - 'a']);
        }

        bool startsWith(std::string prefix) {
            return startsWithRec(prefix);
        }

        bool startsWithRec(std::string prefix) {
            auto node = root;
            return dfs2(prefix, 0, node);
        }

        bool dfs2(std::string& word, int j, TrieNode* node) {
            if(!node) {
                return false;
            }

            if(j == word.size()) {
                return true;
            }

            return dfs2(word, j + 1, node->children[word[j] - 'a']);
        }

    private:
        TrieNode* root;
};

// AC(すごく遅い: 278 ms)
// std::unorderd_map<U,V>を使って読みやすくした別解
namespace ans_2 {
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

    class Trie {
        public:
            Trie() {
                root = new Node();
            }

            void insert(std::string word) {
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
                    if(!(node->children[c])) {
                        return false;
                    }

                    node = node->children[c];
                }

                return node->isWord;
            }

            bool startsWith(std::string prefix) {
                auto node = root;

                for(auto& c: prefix) {
                    if(!(node->children[c])) {
                        return false;
                    }
                    node = node->children[c];
                }

                return true;
            }

        private:
            Node* root;
    };
}

int main(void) {
    auto trie_1 = new TrieAns();
    trie_1->insert("apple");
    std::cout << trie_1->search("apple") << std::endl; 
    // true
    std::cout << trie_1->search("app") << std::endl; 
    // false
    std::cout << trie_1->startsWith("app") << std::endl; 
    // true
    trie_1->insert("app"); 
    std::cout << trie_1->search("app") << std::endl; 
    // true

    ans_2::Trie trie_2;
    trie_2.insert("apple");
    std::cout << trie_2.search("apple") << std::endl; 
    // true
    std::cout << trie_2.search("app") << std::endl; 
    // false
    std::cout << trie_2.startsWith("app") << std::endl; 
    // true
    trie_2.insert("app"); 
    std::cout << trie_2.search("app") << std::endl; 
    // true

    TrieAnsRec trie_rec;
    trie_rec.insert("apple");
    std::cout << trie_rec.searchRec("apple") << std::endl;
    // true
    std::cout << trie_rec.searchRec("app") << std::endl;
    // false
    std::cout << trie_rec.startsWithRec("app") << std::endl;
    // true
}
