class Solution {
public:
    string minWindow(string s, string t) {
        int minWSize = INT_MAX;
        int minWStart = -1;
        int minWEnd = -1;
        unordered_map<char, int> tmap;
        unordered_map<char, int> wmap;
        for (int i = 0; i < t.size(); ++i) {
            int count = 1;
            if (tmap.find(t[i]) != tmap.end()) {
                count += tmap[t[i]];
            }
            tmap[t[i]] = count;
        }

        int n = s.size();
        int l = 0;
        int r = -1;
        while (l < n && r < n) {
            while (!match(wmap, tmap)) {
                r++;
                if (r > n - 1) {
                    break;
                }
                char c = s[r];
                if (tmap.find(c) != tmap.end()) {
                    int count = 1;
                    if (wmap.find(c) != wmap.end()) {
                        count += wmap[c];
                    }
                    wmap[c] = count;
                }
            }
            if (match(wmap, tmap)) {
                if (minWSize > r - l + 1) {
                    minWSize = r - l + 1;
                    minWStart = l;
                    minWEnd = r;
                }
                char c = s[l];
                if (tmap.find(c) != tmap.end()) {
                    int count = wmap[c];
                    if (count - 1 == 0) {
                        wmap.erase(c);
                    } else {
                        wmap[c] = count - 1;
                    }
                }
                l++;
            }
        }
        if (minWStart == -1) return "";
        return s.substr(minWStart, minWEnd + 1 - minWStart);
    }
    bool match(unordered_map<char, int>& wmap, unordered_map<char, int>& tmap) {
        for (auto it = tmap.begin(); it != tmap.end(); ++it) {
            char key = it->first;
            if (wmap.find(key) == wmap.end()) return false;
            if (wmap[key] < it->second) return false;
        }
        return true;
    }
};
