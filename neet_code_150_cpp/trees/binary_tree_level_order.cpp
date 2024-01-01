#include <iostream>
#include <vector>
#include <queue>

// Tree
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

void show_tree_depth(TreeNode* root) {
    if(!root) {
        return;
    }

    std::cout << root->val << std::endl;;

    // left
    show_tree_depth(root->left);
    // right
    show_tree_depth(root->right);
}

// 解けなかった
class Solution {
    template<typename T>
    using Matrix = std::vector<std::vector<T>>;

    public:
         Matrix<int> levelOrder(TreeNode* root) {
            dfs(root);
            return memo;
        }

        std::vector<std::vector<int>> memo;

        void dfs(TreeNode* root) {
            if(!root) {
                return;
            }

            memo.push_back(std::vector{root->val});
            dfs(root->left);
            dfs(root->right);
        }
};

// 模範解答
class SolutionAns {
    public:
        std::vector<std::vector<int>> levelOrder(TreeNode* root) {
            std::vector<std::vector<int>> result;

            if (root == NULL) {
                return result;
            }

            // 幅優先探索
            std::queue<TreeNode*> q;
            q.push(root);

            while (!q.empty()) {
                int count = q.size();
                std::vector<int> curr;

                for (int i = 0; i < count; i++) {
                    TreeNode* node = q.front();
                    q.pop();

                    curr.push_back(node->val);

                    if (node->left != NULL) {
                        q.push(node->left);
                    }
                    if (node->right != NULL) {
                        q.push(node->right);
                    }
                }

                result.push_back(curr);
            }

            return result;
        }
};

int main(void) {
    auto root_1 = TreeNode(3);
    auto root_1_1 = TreeNode(9);
    auto root_1_2 = TreeNode(20);
    auto root_1_3 = TreeNode(15);
    auto root_1_4 = TreeNode(7);
    root_1.left = &root_1_1;
    root_1.right = &root_1_2;
    root_1_2.left = &root_1_3;
    root_1_2.right = &root_1_4;
    //show_tree_depth(&root_1);
    /*
      3
     / \
    /   \
   9    20
       / \
      /   \
     15    7
     */
    // [[3], [9, 20], [15,7]]
    /*
    Solution s_1;
    auto res_1 = s_1.levelOrder(&root_1);
    for(const auto &v: res_1) {
        for(const auto &w: v) {
            std::cout << w << " ";
        }
        std::cout << std::endl;
    }
    */

    SolutionAns s_ans;
    auto res_ans_1 = s_ans.levelOrder(&root_1);
    for(const auto &a: res_ans_1) {
        for(const auto &b: a) {
            std::cout << b << " ";
        }
        std::cout << std::endl;
    }
}
