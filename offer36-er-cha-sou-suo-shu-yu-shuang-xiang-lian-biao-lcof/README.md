# [剑指 Offer 36. 二叉搜索树与双向链表](https://leetcode.cn/problems/er-cha-sou-suo-shu-yu-shuang-xiang-lian-biao-lcof/)

输入一棵二叉搜索树，将该二叉搜索树转换成一个排序的循环双向链表。要求不能创建任何新的节点，只能调整树中节点指针的指向。

 

为了让您更好地理解问题，以下面的二叉搜索树为例：

 ![](https://assets.leetcode.com/uploads/2018/10/12/bstdlloriginalbst.png)

我们希望将这个二叉搜索树转化为双向循环链表。链表中的每个节点都有一个前驱和后继指针。对于双向循环链表，第一个节点的前驱是最后一个节点，最后一个节点的后继是第一个节点。

下图展示了上面的二叉搜索树转化成的链表。“head” 表示指向链表中有最小元素的节点。

 

![](https://assets.leetcode.com/uploads/2018/10/12/bstdllreturndll.png)

 

特别地，我们希望可以就地完成转换操作。当转化完成以后，树中节点的左指针需要指向前驱，树中节点的右指针需要指向后继。还需要返回链表中的第一个节点的指针。

 

**注意：**本题与主站 426 题相同：https://leetcode-cn.com/problems/convert-binary-search-tree-to-sorted-doubly-linked-list/



**注意：**此题对比原题有改动。

**思路**

1、遇到这样的题，首先需要设计好递归函数

2、像这个问题，对于 **左子树** 和 **右子树**，需要知道双向链表的 **头节点** 和 **尾节点**，所以递归的时候需要返回 **两个值**，于是可以直接采用函数指针进行返回，由于二叉树的节点本身就是指针，所以需要传 **二级指针**

3、递归计算左子树变成双向链表的情况

4、递归计算右子树变成双向链表的情况

5、将左子树的双向链表链接到 root 左边，将右子树的双向链表链接到 root 右边，然后根据递归函数的实际作用，返回 **头节点** 和 **尾节点**

**题解**

```c++
// Definition for a Node.
class Node {
public:
    int val;
    Node *left;
    Node *right;

    Node() {}

    Node(int _val) {
        val = _val;
        left = NULL;
        right = NULL;
    }

    Node(int _val, Node *_left, Node *_right) {
        val = _val;
        left = _left;
        right = _right;
    }
};

class Solution {
    void dfs(Node *root, Node **minNode, Node **maxNode) {
        if (root == nullptr) {
            *minNode = nullptr;
            *maxNode = nullptr;
            return;
        }
        Node *lminNode, *lmaxNode, *rminNode, *rmaxNode;
        if (root->left) {
            dfs(root->left, &lminNode, &lmaxNode);
            lmaxNode->right = root;
            root->left = lmaxNode;
            *minNode = lminNode;
        } else {
            *minNode = root;
        }

        if (root->right) {
            dfs(root->right, &rminNode, &rmaxNode);
            rminNode->left = root;
            root->right = rminNode;
            *maxNode = rmaxNode;
        } else {
            *maxNode = root;
        }
    }
public:
    Node *treeToDoublyList(Node *root) {
        if (root == nullptr) {
            return nullptr;
        }
        Node *minNode, *maxNode;
        dfs(root, &minNode, &maxNode);
        maxNode->right = minNode;
        minNode->left = maxNode;
        return minNode;
    }
};
```

