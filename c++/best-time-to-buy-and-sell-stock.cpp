#include <vector>
#include <algorithm>

using std::vector;

class Solution {
public:
    int maxProfit(vector<int>& prices) {
        vector<int> max_sell = prices;
        for(int i = prices.size() - 2; i >= 0; i--){
            if (max_sell[i + 1] > max_sell[i])
                max_sell[i] = max_sell[i + 1];
        }

        int max = 0;
        for (int i = 0; i < prices.size() - 1; i ++){
            int profit = max_sell[i + 1] - prices[i];
            if (profit > max)
                max = profit;
        }

        return max;
    }
};