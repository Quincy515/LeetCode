class Solution {
public:
    vector<int> findAnagrams(string s, string p) {
        int n = s.size();
        int m = p.size();
        if (m > n) return {};
        vector<int> needs(26);
        for (int i = 0; i < m; ++i) {
            needs[p[i] - 'a']++;
        }
        vector<int> matched(26);
        int startp = 0;
        int endp = 0;
        vector<int> result;
        while (endp < m) {
            matched[s[endp] - 'a']++;
            endp++;
        }
        if (same(needs, matched)) {
            result.push_back(startp);
        }
        while (endp < n && startp < n) {
            matched[s[startp] - 'a']--;
            matched[s[endp] - 'a']++;
            startp++;
            endp++;
            if (same(needs, matched)) {
                result.push_back(startp);
            }
        }
        return result;
    }
    bool same(const vector<int>& needs, const vector<int>& matched) {
        for (int i = 0; i < needs.size(); ++i) {
            if (needs[i] != matched[i]) return false;
        }
        return true;
    }
};