class Solution {
public:
    vector<string> result;
    vector<string> letterCombinations(string digits) {
        if (digits.size() == 0) return {};
        vector<string> mappings(10);
        mappings[2] = "abc";
        mappings[3] = "def";
        mappings[4] = "ghi";
        mappings[5] = "jkl";
        mappings[6] = "mno";
        mappings[7] = "pqrs";
        mappings[8] = "tuv";
        mappings[9] = "wxyz";
        vector<char> path(digits.size());
        backtrack(mappings, digits, 0, path);
        return result;
    }
    void backtrack(vector<string> mappings, string digits, int k, vector<char> path) {
        if (k == digits.size()) {
            string str(path.begin(), path.end());
            result.push_back(str);
            return;
        }
        string mapping = mappings[digits[k] - '0'];
        for (int i = 0; i < mapping.size(); ++i) {
            path[k] = mapping[i];
            backtrack(mappings, digits, k + 1, path);
        }
    }
};