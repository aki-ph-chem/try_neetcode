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

class Solution {
    public:
        int kthSmallest(TreeNode* root, int k) {
            if(!root) {
            }

            if(k == 0) {
                return root->val;
            }

            return std::min(
                    kthSmallest(root->left, k - 1),
                    kthSmallest(root->right, k - 1)
                    );
        }
};

// 模範解答
class SolutionAns {
    public:
        int kthSmallest(TreeNode* root, int k) {
            int result = 0;
            inorder(root, k, result);
            return result;
        }

    private:
        void inorder(TreeNode* root, int& k, int& result) {
            if (root == NULL) {
                return;
            }
            inorder(root->left, k, result);
            k--;
            if (k == 0) {
                result = root->val;
                return;
            }
            inorder(root->right, k, result);
        }
};

int main(void) {
    TreeNode root_1(3);
    TreeNode root_1_1(1);
    TreeNode root_1_2(4);
    TreeNode root_1_3(2);
    root_1.left = &root_1_1;
    root_1.right = &root_1_2;
    root_1_1.right = &root_1_3;
    /*
       3
      / \
     /   \
    1     4 
     \
      2

     */
    //k = 1 => 1
    //show_tree_depth(&root_1);

    TreeNode root_2(5);
    TreeNode root_2_1(3);
    TreeNode root_2_2(6);
    TreeNode root_2_3(2);
    TreeNode root_2_4(4);
    TreeNode root_2_5(1);
    root_2.left = &root_2_1;
    root_2.right = &root_2_2;
    root_2_1.left = &root_2_3;
    root_2_1.right = &root_2_4;
    root_2_3.left = &root_2_5;
    /*
              5
             / \
            /   \
           3     6 
          / \
         /   \
        2     4
       /
      /
     1

     */ 
    //k = 3 => 3 
    //show_tree_depth(&root_2);

    /*
    Solution s_1;
    std::cout << s_1.kthSmallest(&root_1, 1) << std::endl; 
    std::cout << s_1.kthSmallest(&root_2, 3) << std::endl; 
    */

    SolutionAns s_ans;
    std::cout << s_ans.kthSmallest(&root_1, 1) << std::endl; 
    std::cout << s_ans.kthSmallest(&root_2, 3) << std::endl; 
}
