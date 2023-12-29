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
        bool isBalanced(TreeNode* root) {
            if(!root) {
                return false;
            }

            return std::abs(isBalanced(root->left) - isBalanced(root->right)) <= 1;
        }

        int depth(TreeNode* root) {
            if(!root) {
                return 0;
            }

            return 1 + depth(root->left); 
        }

};

// 模範解答
class SolutionAns {
    public:
        bool isBalanced(TreeNode* root) {
            int height = 0;
            return dfs(root, height);
        }
    private:
        bool dfs(TreeNode* root, int& height) {
            if (root == NULL) {
                height = -1;
                return true;
            }

            int left = 0;
            int right = 0;

            if (!dfs(root->left, left) || !dfs(root->right, right)) {
                return false;
            }
            if (abs(left - right) > 1) {
                return false;
            }

            height = 1 + std::max(left, right);
            return true;
        }
};

int main(void) {
    TreeNode root_1(3);
    TreeNode root_1_1(9);
    TreeNode root_1_2(20);
    TreeNode root_1_3(15);
    TreeNode root_1_4(7);
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
       /  \
      /    \
     15     7

     => true
     */

    TreeNode root_2(1);
    TreeNode root_2_1(2);
    TreeNode root_2_2(2);
    TreeNode root_2_3(3);
    TreeNode root_2_4(3);
    TreeNode root_2_5(4);
    TreeNode root_2_6(4);
    root_2.left = &root_2_1;
    root_2.right = &root_2_2;
    root_2_1.left = &root_2_3;
    root_2_1.right = &root_2_4;
    root_2_3.left = &root_2_5;
    root_2_3.right = &root_2_6;
    show_tree_depth(&root_2);

    /*
     
          1
         / \
        /   \
       2     2
      / \
     /   \
    3     3
   / \
  /   \
 4     4

     => false
     */
    Solution s_1;
    std::cout << "case_1 " << s_1.isBalanced(&root_1) << std::endl;
    std::cout << "case_2 " << s_1.isBalanced(&root_2) << std::endl;

    SolutionAns s_ans;
    std::cout << "case_1 " << s_ans.isBalanced(&root_1) << std::endl;
    std::cout << "case_2 " << s_ans.isBalanced(&root_2) << std::endl;
}
