impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut res_v: Vec<char> = Vec::new();
        let mut longest = 0;

        let s: Vec<char> = s.chars().collect();

        for i in 0..s.len() {
            let mut left = i;
            let mut right = i;
            while right < s.len() && s[left] == s[right] {
                if longest < right - left + 1 {
                    longest = right - left + 1;
                    let res = &s[left..right + 1];
                    res_v = res.to_vec();
                }
                if left == 0 {
                    break;
                } else {
                    left -= 1;
                }
                right += 1;
            }

            let mut left = i;
            let mut right = i + 1;
            while right < s.len() && s[left] == s[right] {
                if longest < right - left + 1 {
                    longest = right - left + 1;
                    let res = &s[left..right + 1];
                    res_v = res.to_vec();
                }
                if left == 0 {
                    break;
                } else {
                    left -= 1;
                }
                right += 1;
            }
        }

        let result_string: String = res_v.into_iter().collect();
        result_string
    }
}
