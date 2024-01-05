#include <iostream>
#include <vector>
#include <unordered_map>
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

void show_graph(Node* root) {
    std::cout << root->val << std::endl;

    for(const auto &v: root->neighbors) {
        show_graph(v);
    }
}

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

    /*
    Solution s_1;
    auto res_1 = s_1.cloneGraph(&node_1);
    auto res_2 = s_1.cloneGraph(&node_2);

    show_graph(res_1);
    show_graph(res_2);
    */
}
