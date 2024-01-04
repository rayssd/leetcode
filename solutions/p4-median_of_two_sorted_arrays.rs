fn merge_sorted_vectors(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(v1.len() + v2.len());
    let mut i = 0;
    let mut j = 0;

    while i < v1.len() && j < v2.len() {
        if v1[i] < v2[j] {
            result.push(v1[i]);
            i += 1;
        } else {
            result.push(v2[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&v1[i..]);
    result.extend_from_slice(&v2[j..]);

    result
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let result = merge_sorted_vectors(nums1, nums2);
        let length = result.len();
        let mut median: f64 = 0.0;

        if length % 2 == 0 {
            median = (result[length / 2] as f64 + (result[length / 2 - 1]) as f64) / 2.0;
        } else {
            median = result[length / 2] as f64;
        }

        median
    }
}
