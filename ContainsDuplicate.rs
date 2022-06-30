use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut visited = HashSet::new();
        
        for num in &nums {
            if visited.contains(num){
                return true
            }
            visited.insert(num);
        }
        return false;
    }
}
