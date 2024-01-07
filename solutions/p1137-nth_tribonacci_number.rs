impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let n = n as usize;
        let mut tri_fib: Vec<i32> = vec![0;n+3];

        tri_fib[0] = 0;
        tri_fib[1] = 1;
        tri_fib[2] = 1;

        if n >= 3 {
            for i in 3..n+1 {
                tri_fib[i] = tri_fib[i-1] + tri_fib[i-2] + tri_fib[i-3]; 
            }
        }

        return tri_fib[n]; 
    }
}
