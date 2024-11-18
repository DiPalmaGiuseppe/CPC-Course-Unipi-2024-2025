// https://leetcode.com/problems/maximum-subarray/description/

int maxSubArray(int* nums, int numsSize) {
    int max = nums[0];
    int cont = 0;

    for(int i = 0; i<numsSize;i++){
        cont += nums[i];
        if(cont > max)
            max = cont;
        if(cont <= 0)
            cont = 0;
    }
    return max;
}