impl Solution {
    pub fn fib(n: i32) -> i32 {
        let n = n as usize;
        let mut fib: Vec<i32> = vec![0;n+1];

        fib[0] = 0;
        if n >=1 {
            fib[1] = 1;
            for i in 2..n+1 {
            fib[i] = fib[i-1] + fib[i-2]; 
            }
        }

        return fib[n];    
    }
}
