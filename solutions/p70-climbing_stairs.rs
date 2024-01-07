impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut fib: Vec<i32> = vec![0;n+2];

        fib[0] = 0;
        fib[1] = 1;

        for i in 2..n+2 {
           fib[i] = fib[i-1] + fib[i-2]; 
        }

        return fib[n+1];

        
    }
}
