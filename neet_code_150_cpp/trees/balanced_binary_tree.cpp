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

class Solution {
    public:
        bool isBalanced(TreeNode* root) {
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
    show_tree_depth(&root_1);
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
    root_1.left = &root_1_1;
    root_1.right = &root_1_2;
    root_1_2.left = &root_1_3;
    root_1_2.right = &root_1_4;
    show_tree_depth(&root_1);

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
}
