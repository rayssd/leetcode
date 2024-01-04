impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in (0..nums.len()) {
            let res = target - nums[i];
            for j in (i + 1..nums.len()) {
                if res == nums[j] {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![0 as i32, 0 as i32]
    }
}
