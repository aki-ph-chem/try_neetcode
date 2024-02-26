#include <climits>
#include <iostream>

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

// 解けなかった
class Solution {
    public:
        int maxPathSum(TreeNode* root) {

            return 1;
        }
};

// 模範解答
class SolutionAns {
    public:
        int maxPathSum(TreeNode* root) {
            auto maxPath = INT_MIN;
            dfs(root, maxPath);

            return maxPath;
        }

    private:
        int dfs(const TreeNode* root, int& maxPath) {
            if(!root) {
                return 0;
            }

            auto left = std::max(dfs(root->left, maxPath), 0);
            auto right = std::max(dfs(root->right, maxPath), 0);

            auto currentPath = root->val + left + right;
            maxPath = std::max(maxPath, currentPath);

            return root->val + std::max(left, right);
        }
};

void show_tree_dfs(const TreeNode* root) {
    if(!root) {
        return;
    }

    std::cout << root->val << std::endl;;

    // left
    show_tree_dfs(root->right);
    // right
    show_tree_dfs(root->left);
}


int main(void) {
    TreeNode root_1(1);
    TreeNode root_1_1(2);
    TreeNode root_1_2(3);
    root_1.right = &root_1_1;
    root_1.left = &root_1_2;
    //show_tree_dfs(&root_1);
    // => 6 (2 + 1 + 3)

    TreeNode root_2(-10);
    TreeNode root_2_1(9);
    TreeNode root_2_2(20);
    TreeNode root_2_3(15);
    TreeNode root_2_4(7);
    root_2.right = &root_2_1;
    root_2.left = &root_2_2;
    root_2_2.right = &root_2_3;
    root_2_2.left = &root_2_4;
    //show_tree_dfs(&root_2);
    // => 42 (15 + 20 + 7)

    SolutionAns s_ans;
    std::cout << s_ans.maxPathSum(&root_1) << std::endl;
    std::cout << s_ans.maxPathSum(&root_2) << std::endl;
}
