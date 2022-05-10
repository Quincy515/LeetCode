class Solution {
public:
    bool isPowerOfTwo(int n) {
        while (n != 0) {
            if ((n & 1) == 1) {
                if ((n >> 1) == 0) {
                    return true;
                } else {
                    return false;
                }
            }
            n >>= 1;
        }
        return false;
    }
};
