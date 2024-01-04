impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let radix: i32 = 10;
        let mut reversed: i32 = 0;
        while x != 0 {
            match reversed.checked_mul(radix) {
                Some(p) => {
                    reversed = p + x % radix;
                    x /= radix;
                }
                None => {
                    return 0;
                }
            }
        }

        return reversed;
    }
}
