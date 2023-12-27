#include <iostream>

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
    /*
     root_1

        1
       / \
      /   \
     3     5 
    / \
   /   \
  2     4
     
     */
    show_tree_depth(&root_1);
}
