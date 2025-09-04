#include <vector>
using std::vector;
class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int totalProfit = 0;
        int buy = 0, sell = 0;
        while (sell < prices.size()){
            totalProfit += prices[sell] - prices[buy];
            buy = sell;
            while (buy <= (int)prices.size() - 2 && prices[buy] >= prices[buy + 1])
                buy++;
            sell = buy + 1;
            while (sell < (int)prices.size() - 1 && prices[sell] < prices[sell + 1])
                sell++;
        }

        return totalProfit;
    }
};