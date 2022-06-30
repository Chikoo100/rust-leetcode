use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false
        }
        
        let mut visited : HashMap<char, i32> = HashMap::new();
        
        for c in s.chars() {
            let val = visited.entry(c).or_insert(0);
            *val += 1;
        }
        
        for c in t.chars() {
            let val = visited.entry(c).or_insert(0);
            *val -= 1;
        }

        for (k,v) in &visited {
            if *v != 0 {
                return false;
            }
        }
        
        true
    }
}
