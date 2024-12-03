//https://leetcode.com/problems/continuous-subarray-sum/

use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(mut nums: Vec<i32>, mut k: i32) -> bool {

        if nums.len() < 2 {
            return false;
        }

        let prefix_sum = nums.iter().scan(0, |sum, i| {*sum += i; Some(*sum)}).collect::<Vec<_>>();
        let mut reminders = HashMap::new();
        for (i, val) in prefix_sum.iter().enumerate() {
            let r = val % k;

            if r == 0 && i!=0 {
                return true;
            }
            if let Some(&prev_index) = reminders.get(&r) {
                if i - prev_index > 1{
                    return true;
                }
            } else {
                reminders.insert(r, i);
            }
        }

        return false
    }
}