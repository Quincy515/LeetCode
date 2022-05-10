class Solution {
public:
    vector<vector<string>> result;
    vector<vector<string>> partition(string s) {
        vector<string> path;
        backtrack(s, 0, path);
        return result;
    }
    void backtrack(string s, int k, vector<string>& path) {
        if (k == s.size()) {
            result.push_back(path);
            return;
        }
        for (int end = k; end < s.size(); ++end) {
            if (ispalindrome(s, k, end)) {
                path.push_back(s.substr(k, end + 1 - k));
                backtrack(s, end + 1, path);
                path.pop_back();
            }
        }
    }
    bool ispalindrome(string s, int p, int r) {
        int i = p;
        int j = r;
        while (i <= j) {
            if (s[i] != s[j]) return false;
            i++;
            j--;
        }
        return true;
    }
};