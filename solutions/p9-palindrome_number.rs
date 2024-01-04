impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            return false;
        } else if x < 10 {
            return true;
        }
        let original = x.clone();

        let radix: i32 = 10;
        let mut reversed: i32 = 0;
        while x != 0 {
            match reversed.checked_mul(radix) {
                Some(p) => {
                    reversed = p + x % radix;
                    x /= radix;
                }
                None => {
                    return false;
                }
            }
        }

        return reversed == original;
    }
}
