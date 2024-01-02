#include <iostream>
#include <string>

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
}
