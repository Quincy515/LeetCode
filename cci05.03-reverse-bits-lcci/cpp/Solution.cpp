class Solution {
public:
    int reverseBits(int num) {
        if (num == 0) return 1;
        std::vector<int> nums(32);
        for (int i = 0; i < 32; ++i) {
            nums[i] = (num & 1);
            num >>= 1;
        }
        vector<int> leftOneCounts(32);
        int count = 0;
        for (int i = 0; i < 32; ++i) {
            leftOneCounts[i] = count;
            if (nums[i] == 1) {
                count++;
            } else {
                count = 0;
            }
        }
        vector<int> rightOneCounts(32);
        count = 0;
        for (int i = 31; i >= 0; --i) {
            rightOneCounts[i] = count;
            if (nums[i] == 1) {
                count++;
            } else {
                count = 0;
            }
        }
        int ret = 0;
        for (int i = 0; i < 32; ++i) {
            if (ret < leftOneCounts[i] + rightOneCounts[i] + 1) {
                ret = leftOneCounts[i] + rightOneCounts[i] + 1;
            }
        }
        return ret;
    }
};