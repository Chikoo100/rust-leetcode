use std::collections::HashMap;
impl Solution {
    pub fn is_valid(s: String) -> bool {

        let mut stack : Vec<char> = Vec::new();
        let pairs : HashMap<char, char>= HashMap::from([
            ('(', ')'),
            ('[', ']'),
            ('{', '}')
        ]);

        for c in s.chars() {
            if pairs.contains_key(&c) {
                stack.push(c);
            }
            else {
                match stack.pop() {
                    Some(x) => match pairs.get(&x) {
                        Some(a) => if c != *a {
                        return false;
                    }
                        _ => return false,
                    }
                    _ => return false,
                }
            }
        }

        if stack.len() == 0 {
            return true;
        }

        false

    }
}
