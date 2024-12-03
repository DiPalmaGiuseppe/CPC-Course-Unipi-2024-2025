// https://leetcode.com/problems/check-if-all-the-integers-in-a-range-are-covered/submissions/1460198766/

class Solution {
public:
    bool isCovered(vector<vector<int>>& ranges, int left, int right) {
        vector<int> vec(50);
        for (auto r: ranges) {
            int i=r[0]-1;
            if(i < 50)
                vec[i]+=1;
            i=r[1]-1;
            if(i < 49)
                vec[i+1]-=1;
        }
        int active = 0;
        int i=0;
        for(auto v: vec){
            active += v;
            vec[i] = active;
            if(i>=left-1 && i<=right-1){
                if(vec[i] == 0)
                    return false;   
            }
            i++;
        }
        return true;
    }
};