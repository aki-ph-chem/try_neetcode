#include <iostream>
#include <vector>
#include <unordered_map>
#include <unordered_set>
#include <queue>


// Nodeの定義
class Node {
    public:
        int val;
        std::vector<Node*> neighbors;

        Node() {
            val = 0;
            neighbors = std::vector<Node*>();
        }

        Node(int _val) {
            val = _val;
            neighbors = std::vector<Node*>();
        }

        Node(int _val, std::vector<Node*> _neighbors) {
            val = _val;
            neighbors = _neighbors;
        }
};

// 解けなかった
class Solution {
    public:
        Node* cloneGraph(Node* node) {
            auto root = new Node(node->val);

            if(!node->neighbors.size()) {
                return root;
            }

            std::vector<Node*> ngh;
            for(auto &v: node->neighbors) {
                auto c = new Node(v->val);
                ngh.push_back(c);
                v = cloneGraph(v);
            }

            root->neighbors = ngh;
            return root;
        }
};

// 模範解答
class SolutionAns {
    public:
        Node* cloneGraph(Node* node) {
            if (node == NULL) {
                return NULL;
            }

            Node* copy = new Node(node->val);
            m[node] = copy;

            std::queue<Node*> q;
            q.push(node);

            while (!q.empty()) {
                Node* curr = q.front();
                q.pop();

                for (int i = 0; i < curr->neighbors.size(); i++) {
                    Node* neighbor = curr->neighbors[i];

                    if (m.find(neighbor) == m.end()) {
                        m[neighbor] = new Node(neighbor->val);
                        q.push(neighbor);
                    }

                    m[curr]->neighbors.push_back(m[neighbor]);
                }
            }

            return copy;
        }
    private:
        std::unordered_map<Node*, Node*> m;
};

// 模範解答を少しスッキリさせた
// AC
class SolutionAns2 {
#define CXX_20

    public:
        Node* cloneGraph(Node* node) {
            if(!node) {
                return nullptr;
            }

            auto copy = new Node(node->val);
            m.insert({node, copy});

            // キューを使った深さ優先探索
            std::queue<Node*> q;
            q.push(node);
            while(!q.empty()) {
                auto current = q.front();
                q.pop();

                for(const auto &v: current->neighbors) {
#ifndef CXX_20
                    // vが見つからなかった場合
                    if(m.find(v) == m.end()) {
#else
                        if(!m.contains(v)) {
#endif
                            m.insert({v, new Node(v->val)});
                            q.push(v);
                        }

                        m[current]->neighbors.push_back(m[v]);
                    }
                }

                return copy;
            }

            private:
            // オリジナルのNodeとコピーのNodeのペア
            std::unordered_map<Node*, Node*> m;
        };

int main(void) {
    Node node_1 = Node(1);
    Node node_1_1 = Node(2);
    Node node_1_2 = Node(3);
    node_1.neighbors = std::vector{&node_1_1, &node_1_2};
    //show_graph(&node_1);

    Node node_2 = Node(1);
    Node node_2_1 = Node(2);
    Node node_2_2 = Node(3);
    node_2.neighbors = std::vector{&node_2_1, &node_2_2};
    node_2_1.neighbors = std::vector{&node_2_2};

    Node node_3 = Node(1);
    Node node_3_1 = Node(2);
    Node node_3_2 = Node(3);
    Node node_3_3 = Node(4);
    node_3.neighbors = std::vector{&node_3_1, &node_3_3};
    node_3_1.neighbors = std::vector{&node_3, &node_3_2};
    node_3_2.neighbors = std::vector{&node_3_1, &node_3_3};
    node_3_3.neighbors = std::vector{&node_3, &node_3_2};

    /*
    Solution s_1;
    auto res_1 = s_1.cloneGraph(&node_1);
    auto res_2 = s_1.cloneGraph(&node_2);

    */

    SolutionAns s_ans;
    auto res_1_ans = s_ans.cloneGraph(&node_1);
    auto res_2_ans = s_ans.cloneGraph(&node_2);

    SolutionAns2 s_ans_2;
    auto res_1_ans_2 = s_ans_2.cloneGraph(&node_1);
    auto res_2_ans_2 = s_ans_2.cloneGraph(&node_2);
}
