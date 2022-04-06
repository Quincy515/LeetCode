class Solution {
public:
    int exchangeBits(int num) {
        int ret = 0;
        for (int i = 0; i <= 30; i += 2) {
            int a1 = (num & (1 << i));
            int b1 = (num & (1 << (i + 1)));
            if (a1 != 0) ret |= (1 << (i + 1));
            if (b1 != 0) ret |= (1 << i);
        }
        return ret;
    }
};
