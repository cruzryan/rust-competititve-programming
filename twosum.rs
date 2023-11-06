// From LeetCode: Two Sum

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_map: HashMap<i32, i32> = HashMap::new();

        for (i, eli) in nums.iter().enumerate() {
            let complement = target - eli;
            if let Some(&index) = nums_map.get(&complement) {
                return vec![index as i32, i as i32];
            }
        
            nums_map.insert(*eli, i as i32);
        }

        vec![0,0]
    }
}


fn main() {
    let nums = vec![3,2,4];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    println!("result: {:?}", result);
}