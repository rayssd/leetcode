use std::collections::HashMap;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut char_map: HashMap<char, usize> = HashMap::new();

        for i in 0..words.len() {
            for (_, c) in words[i].chars().enumerate() {
                char_map
                    .entry(c)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        }

        for (_, count) in char_map.iter() {
            if count % words.len() != 0 {
                return false;
            }
        }

        return true;
    }
}
