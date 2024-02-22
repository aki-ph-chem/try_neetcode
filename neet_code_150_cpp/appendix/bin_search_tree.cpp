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

// 単純なdfsによる探索
// O(N)
bool is_contain(const TreeNode* root,int x) {
    if(!root) {
        return false;
    }

    if(root->val == x) {
        return true;
    }

    return is_contain(root->left, x) || is_contain(root->right, x);
}

// 二分探索木の特徴を活かした探索
// O(log(N))
bool is_contain_bin_search_tree(const TreeNode* root, int x) {
    if(!root) {
        return false;
    }

    if(root->val == x) {
        return true;
    }

    if(x < root->val) {
        return is_contain_bin_search_tree(root->left, x);
    }

    return is_contain_bin_search_tree(root->right, x);
}

// dfsの順番で要素を表示
void show_dfs(const TreeNode* root) {
    if(!root) {
        return;
    }

    std::cout << root->val << '\n';

    show_dfs(root->left);
    show_dfs(root->right);
}

int main(void) {
    auto root_1 = TreeNode(5);
    auto root_1_1 = TreeNode(3);
    auto root_1_2 = TreeNode(8);
    auto root_1_3 = TreeNode(1);
    auto root_1_4 = TreeNode(4);
    auto root_1_5 = TreeNode(10);

    root_1.left = &root_1_1;
    root_1.right = &root_1_2;
    root_1_1.left = &root_1_3;
    root_1_1.right = nullptr;
    root_1_2.left = &root_1_4;
    root_1_2.right = &root_1_5;

    //          5
    //         / \
    //        /   \
    //       3     8
    //      /     / \
    //     /     /   \
    //    1     4    10
    //    

    std::cout << is_contain(&root_1, 8) << std::endl;
    // => true
    std::cout << is_contain(&root_1, 100) << std::endl;
    // => false
    std::cout << is_contain_bin_search_tree(&root_1, 8) << std::endl;
    // => true
    std::cout << is_contain_bin_search_tree(&root_1, 100) << std::endl;
    // => false
    
    show_dfs(&root_1);
}
