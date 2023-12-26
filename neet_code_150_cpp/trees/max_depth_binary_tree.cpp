#include <iostream>
#include <vector>

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
    public:
        int maxDepth(TreeNode* root) {
            if(!root) {
            }

            return 1;
        }

};

// 模範解答
class SolutionAns {
    public:
        int maxDepth(TreeNode* root) {
            if(!root) {
                return 0;
            }

            return 1 + std::max(maxDepth(root->left), maxDepth(root->right));
        }
};

int main(void) {
    TreeNode t_1_0(3);
    TreeNode t_1_1(9);
    TreeNode t_1_2(20);
    TreeNode t_1_3(15);
    TreeNode t_1_4(7);

    t_1_0.left = &t_1_1;
    t_1_1.left = nullptr;
    t_1_1.right = nullptr;
    t_1_0.right = &t_1_2;
    t_1_2.left = &t_1_3;
    t_1_2.right = &t_1_4;
    t_1_3.left = nullptr;
    t_1_3.right = nullptr;
    t_1_4.left = nullptr;
    t_1_4.right = nullptr;
    //
    //    3 
    //   / \
    //  /   \
    // 9    20
    //     /  \
    //    /    \
    //   15     7
    //
    // => 3
    
    //show_tree_depth(&t_1_0);
    SolutionAns s_ans;
    std::cout << s_ans.maxDepth(&t_1_0) << std::endl;
}
