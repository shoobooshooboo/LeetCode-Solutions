#include <algorithm>
#include <vector>

using std::vector;
class Solution {
public:
    int hIndex(vector<int>& citations) {
        vector<int> counts(citations.size() + 1);
        for (const auto &c : citations){
            counts[std::min(c, (int)counts.size() - 1)]++;
        }

        int total = 0;
        for(int i = counts.size() - 1; i > 0; i--){
            total += counts[i];
            if (total >= i)
                return i;
        }
        return 0;
    }
};