#include <iomanip>
#include <iostream>
#include <queue>

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;

    TreeNode()
        :val(0), left(nullptr), right(nullptr) {}

    TreeNode(int x)
        :val(x), left(nullptr), right(nullptr) {}

    TreeNode(int x, TreeNode* left, TreeNode* right)
        :val(x), left(left), right(right) {}
};

void show_tree_dfs(const TreeNode* root) {
    if(!root) {
        return;
    }

    std::cout << root->val << std::endl;;

    // left
    show_tree_dfs(root->left);
    // right
    show_tree_dfs(root->right);
}

bool is_contain_dfs(const TreeNode* root, int x) {
    if(!root) {
        return false;
    }

    if(root->val == x) {
        return true;
    }

    return is_contain_dfs(root->left, x) || is_contain_dfs(root->right, x);
}

void show_tree_bfs(const TreeNode* root) {
    if(!root) {
        return;
    }

    // BFS
    std::queue<const TreeNode*> q;
    q.push(root);

    while(!q.empty()) {
        auto count = q.size();

        for(int i = 0; i < count; ++i) {
            auto node = q.front();
            q.pop();

            std::cout << node->val << " ";

            if(node->left) {
                q.push(node->left);
            }
            if(node->right) {
                q.push(node->right);
            }
        }
        std::cout << '\n';
    }
}

bool is_contain_bfs(const TreeNode* root, int x) {
    if(!root) {
        return false;
    }

    //BFS
    std::queue<const TreeNode*> q;
    q.push(root);

    while(!q.empty()) {
        auto count = q.size();

        for(int i = 0; i < count; ++i) {
            auto node = q.front();
            q.pop();

            if(node->val == x) {
                return true;
            }

            if(node->left) {
                q.push(node->left);
            }
            if(node->right) {
                q.push(node->right);
            }
        }
    }

    return false;
}

int main(void) {
    TreeNode root_1(1);
    TreeNode root_1_1(3);
    TreeNode root_1_2(5);
    TreeNode root_1_3(2);
    TreeNode root_1_4(4);
    root_1.left = &root_1_1;
    root_1.right = &root_1_2;
    root_1_1.left = &root_1_3;
    root_1_1.right = &root_1_4;
   //root_1
   //         1
   //        / \
   //       /   \
   //      3     5 
   //     / \
   //    /   \
   //   2     4
   //
    std::cout << "DFS" << std::endl;
    show_tree_dfs(&root_1);
    std::cout << "BFS" << std::endl;
    show_tree_bfs(&root_1);

    std::cout << is_contain_dfs(&root_1, 4) << std::endl;
    std::cout << is_contain_dfs(&root_1, 30) << std::endl;

    std::cout << is_contain_bfs(&root_1, 4) << std::endl;
    std::cout << is_contain_bfs(&root_1, 30) << std::endl;
}
