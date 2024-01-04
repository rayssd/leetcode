impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut mutable_g = g.clone();
        mutable_g.sort();
        let mut mutable_s = s.clone();
        mutable_s.sort();
        let mut start = 0;

        for i in 0..mutable_g.len() {
            for j in start..mutable_s.len() {
                if mutable_s[j] >= mutable_g[i] {
                    mutable_s[j] = -1; // cookie taken
                    start = j;
                    break;
                }
            }
        }

        mutable_s.retain(|&x| x < 0);
        return mutable_s.len() as i32;
    }
}
