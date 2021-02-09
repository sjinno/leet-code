# 3. Longest Substring Without Repeating Characters
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        
        let mut lg = 0;
        let mut ptr = 0;
        let mut visited = HashMap::<char, usize>::new();
        
        for (i, c) in s.chars().enumerate() {
            // If a letter is in in the string, then we update our pointer `ptr`
            // to the current position.
            if let Some(pos) = visited.get_mut(&c) {
                if *pos > ptr { ptr = *pos; }
            }
            
            // Note that there is +1 because we intentionally set ptr to
            // (the index of char + 1) to keep track of the length easily.
            let curr_lg = i + 1 - ptr;
            
            if curr_lg > lg { 
                lg = curr_lg; 
            }
            
            // +1 because index starts from 0.
            // So in order to keep track of the length
            // until the repeating character, we add 1 for convenience.
            visited.insert(c, i + 1);
        }
        
        lg as i32
    }
}

fn main() {
}