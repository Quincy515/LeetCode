// Definition for a Node.
class Node {
public:
    int val;
    Node *prev;
    Node *next;
    Node *child;
};

class Solution {
    Node *dfs(Node *head) {
        Node *now = head;
        Node *last = nullptr;

        while (now) {
            Node *cLast;
            if (now->child) {
                cLast = dfs(now->child);
                Node *next = now->next;

                now->next = now->child;
                now->child = nullptr;
                now->next->prev = now;

                if (next) {
                    next->prev = cLast;
                }
                cLast->next = next;
            }
            if (now->next == nullptr) {
                if (now->child) {
                    last = cLast;
                } else {
                    last = now;
                }
            }
            now = now->next;
        }
        return last;
    }

public:
    Node *flatten(Node *head) {
        if (head == nullptr) {
            return nullptr;
        }
        Node *last = dfs(head);
        last->next = nullptr;
        return head;
    }
};