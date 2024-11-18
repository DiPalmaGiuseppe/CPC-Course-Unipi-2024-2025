//  https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/submissions/1456547752/

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
int* searchRange(int* nums, int numsSize, int target, int* returnSize) {
    *returnSize = 2;
    int* position = (int *)malloc(sizeof(int) * (*returnSize));
    
    position[0]=-1;
    position[1]=-1;


    int left = 0, right = numsSize-1, mid;
    while(left<=right){
        mid = left + (right-left)/2;

        if(nums[mid] > target){
            right = mid-1;
            continue;
        }

        if(nums[mid] < target){
            left = mid+1;
            continue;
        }
        
        if(mid==0 || nums[mid-1]!=target){
            position[0]=mid;
            break;
        }
        right=mid-1;
    }

    left = 0; right = numsSize-1;
    while(left<=right){
        mid = left + (right-left)/2;

        if(nums[mid] > target){
            right = mid-1;
            continue;
        }

        if(nums[mid] < target){
            left = mid+1;
            continue;
        }
        
        if(mid==numsSize-1 || nums[mid+1]!=target){
            position[1]=mid;
            break;
        }
        left=mid+1;
    }

    return position;
}