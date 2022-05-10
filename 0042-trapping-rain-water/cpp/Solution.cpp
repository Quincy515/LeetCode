class Solution {
public:
    int trap(vector<int>& height) {
        int n = height.size();
        int result = 0;
        for (int i = 1; i < n - 1; ++i) {
            int lh = 0;
            for (int j = 0; j < i; ++j) {
                if (height[j] > lh) {
                    lh = height[j];
                }
            }
            int rh = 0;
            for (int j = i + 1; j < n; ++j) {
                if (height[j] > rh) {
                    rh = height[j];
                }
            }
            int carry = std::min(lh, rh) - height[i];
            if (carry < 0) carry = 0;
            result += carry;
        }
        return result;
    }
};

class Solution {
public:
    int trap(vector<int>& height) {
        int n = height.size();
        // 前缀
        vector<int> leftMax(n);
        int max = 0;
        for (int i = 0; i < n ; ++i) {
            leftMax[i] = std::max(max, height[i]);
            max = leftMax[i];
        }
        // 后缀
        vector<int> rightMax(n);
        max = 0;
        for (int i = n - 1; i >= 0 ; --i) {
            rightMax[i] = std::max(max, height[i]);
            max = rightMax[i];
        }
        // 每个柱⼦之上承载的⾬⽔
        int result = 0;
        for (int i = 1; i < n - 1; ++i) {
            result += std::min(leftMax[i], rightMax[i]) - height[i];
        }
        return result;
    }
};