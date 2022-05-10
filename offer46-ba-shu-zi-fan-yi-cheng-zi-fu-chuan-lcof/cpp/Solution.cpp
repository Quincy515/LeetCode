class Solution {
public:
    int translateNum(int num) {
        if (num <= 9) return 1;
        std::vector<int> digitlist;
        while (num) {
            digitlist.push_back(num % 10);
            num /= 10;
        }
        int n = digitlist.size();
        vector<int> digits(n);
        for (int i = 0; i < n; ++i) {
            digits[i] = digitlist[n - i - 1];
        }
        vector<int> dp(n + 1);
        dp[0] = 1;
        for (int i = 1; i <= n; ++i) {
            dp[i] = dp[i - 1];
            if (i - 2 >= 0 && isValid(digits[i - 2], digits[i - 1])) {
                dp[i] += dp[i - 2];
            }
        }
        return dp[n];
    }
    bool isValid(int a, int b) {
        if (a == 1) return true;
        if (a == 2 && b >= 0 && b <= 5) return true;
        return false;
    }
};