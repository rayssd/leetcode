use std::cmp;
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_set: HashMap<char, usize> = HashMap::new();
        let mut left = 0;
        let mut current_longest = 0;

        for (i, char) in s.chars().enumerate() {
            if char_set.contains_key(&char) && char_set.get(&char).unwrap() >= &left {
                left = char_set.get(&char).unwrap() + 1;
            } else {
                current_longest = cmp::max(current_longest, i - left + 1);
            }

            *char_set.entry(char).or_insert(i) = i;
        }

        current_longest as i32
    }
}
