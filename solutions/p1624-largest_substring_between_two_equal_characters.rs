impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        use std::cmp;

        let mut start;
        let mut end;
        let mut longest: i32 = -1;

        let s: Vec<char> = s.chars().collect();

        for i in 0..s.len() {
            for j in (i..s.len()).rev() {
                if s[i] == s[j] {
                    start = i;
                    end = j;
                    longest = cmp::max(longest, end as i32 - start as i32 - 1);
                }
            }
        }

        longest
    }
}
