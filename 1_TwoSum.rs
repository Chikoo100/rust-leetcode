use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut compliments : HashMap<i32, i32> = HashMap::new();
        
        for (i, value) in nums.iter().enumerate() {
            match compliments.get(value) {
                Some(&index) => return vec![index, i as i32],
                None => compliments.insert(target - value, i as i32),
            };
        }
        
        return vec![]

    }
    
}
