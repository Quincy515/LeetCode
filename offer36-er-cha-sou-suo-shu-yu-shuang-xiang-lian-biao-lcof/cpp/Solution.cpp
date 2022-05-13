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