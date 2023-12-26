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

// 解けなかった
class Solution {
    public:
        bool isSubtree(TreeNode* root, TreeNode* subRoot) {
            if(!root && !subRoot) {
                return true;
            } else if(!root || !subRoot) {
                return false;
            }

            if(root->val == subRoot->val) {
                return isSamaTree(root, subRoot);
            }

            return isSubtree(root->left, subRoot) && isSubtree(root->right, subRoot);
        }

    private:
        bool isSamaTree(TreeNode* p, TreeNode* q) {
            if(!p && !q) {
                return true;
            } else if(!p || !q) {
                return false;
            }

            if(p->val != q->val) {
                return false;
            }

            return isSamaTree(p->left, q->left) && isSamaTree(p->right, q->right);
        }
};

// 模範解答
class SolutionAns {
    public:
        bool isSubtree(TreeNode* root, TreeNode* subRoot) {
            if (root == NULL) {
                return false;
            }
            if (isSame(root, subRoot)) {
                return true;
            }
            return isSubtree(root->left, subRoot) || isSubtree(root->right, subRoot);
        }

    private:
        bool isSame(TreeNode* root, TreeNode* subRoot) {
            if (root == NULL && subRoot == NULL) {
                return true;
            }
            if (root == NULL || subRoot == NULL) {
                return false;
            }
            if (root->val != subRoot->val) {
                return false;
            }
            return isSame(root->left, subRoot->left) && isSame(root->right, subRoot->right);
        }
};

int main(void) {
    TreeNode root_1(3);
    TreeNode root_1_1(4);
    TreeNode root_1_2(5);
    TreeNode root_1_3(1);
    TreeNode root_1_4(2);
    root_1.left = &root_1_1;
    root_1.right = &root_1_2;
    root_1_1.left = &root_1_3;
    root_1_1.right = &root_1_4;
    TreeNode subRoot_1(4);
    TreeNode subRoot_1_1(1);
    TreeNode subRoot_1_2(2);
    subRoot_1.left = &subRoot_1_1;
    subRoot_1.right = &subRoot_1_2;
   /* 
           root      subRoot 
          
            3         4
           / \       / \
          /   \     /   \
         4     5   1     2 
        / \
       /   \
      1     2 

     =>  true
   */

    TreeNode root_2(3);
    TreeNode root_2_1(4);
    TreeNode root_2_2(5);
    TreeNode root_2_3(1);
    TreeNode root_2_4(2);
    root_1.left = &root_1_1;
    root_1.right = &root_1_2;
    root_1_1.left = &root_1_3;
    root_1_1.right = &root_1_4;
    TreeNode subRoot_2(4);
    TreeNode subRoot_2_1(1);
    TreeNode subRoot_2_2(2);
    subRoot_1.left = &subRoot_1_1;
    subRoot_1.right = &subRoot_1_2;
   /* 
           root      subRoot 
          
            3         4
           / \       / \
          /   \     /   \
         4     5   1     2 
        / \
       /   \
      1     2 
           /
          /
         0

     => false 
   */

    Solution s_1;

    std::cout << s_1.isSubtree(&root_1, &subRoot_1) << std::endl;
    std::cout << s_1.isSubtree(&root_2, &subRoot_2) << std::endl;
}
