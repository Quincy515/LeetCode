class Solution {
public:
    vector<string> result;
    vector<string> restoreIpAddresses(string s) {
        vector<int> path;
        backtrack(s, 0, 0, path);
        return result;
    }
    void backtrack(string s, int k, int step, vector<int> &path) {
        if (k == s.size() && step == 4) {
            string sb;
            for (int i = 0; i < 3; ++i) {
                sb += to_string(path[i]);
                sb += ".";
            }
            sb += to_string(path[3]);
            result.push_back(sb);
            return;
        }
        if (step > 4) {
            return;
        }
        if (k == s.size()) {
            return;
        }
        int val = 0;
        if (k < s.size()) {
            val = val * 10 + (s[k] - '0');
            path.push_back(val);
            backtrack(s, k + 1, step + 1, path);
            path.pop_back();
        }
        if (s[k] == '0') {
            return;
        }
        if (k + 1 < s.size()) {
            val = val * 10 + (s[k + 1] - '0');
            path.push_back(val);
            backtrack(s, k + 2, step + 1, path);
            path.pop_back();
        }
        if (k + 2 < s.size()) {
            val = val * 10 + (s[k+2] - '0');
            if (val <= 255) {
                path.push_back(val);
                backtrack(s, k + 3, step + 1, path);
                path.pop_back();
            }
        }
    }
};