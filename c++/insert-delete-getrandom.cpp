#include <unordered_map>
#include <vector>
#include <algorithm>
#include <cstdlib>
#include <ctime>

class RandomizedSet {
private: 
    std::unordered_map<int, int> indices;
    std::vector<int> values;
public:
    RandomizedSet() {
        std::srand(std::time({}));
    }
    
    bool insert(int val) {
        if (indices.count(val)) return false;
        values.push_back(val);
        indices[val] = values.size() - 1;
        return true;
    }
    
    bool remove(int val) {
        if (!indices.count(val)) return false;
        
        int old_index = indices[val];
        std::swap(values[old_index], values[values.size() - 1]);
        indices[values[old_index]] = old_index;

        values.pop_back();
        indices.erase(val);
        return true;
    }
    
    int getRandom() {
        return values[std::rand() % values.size()];
    }
};

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * RandomizedSet* obj = new RandomizedSet();
 * bool param_1 = obj->insert(val);
 * bool param_2 = obj->remove(val);
 * int param_3 = obj->getRandom();
 */