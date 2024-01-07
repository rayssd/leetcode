struct Job {
    start_time: i32,
    end_time: i32,
    profit: i32
} 

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        // store the 3 vectors together in a struct
                use std::cmp;
        let mut jobs: Vec<Job> = Vec::with_capacity(profit.len());

        for i in 0..profit.len() {
            let job = Job {
                start_time: start_time[i],
                end_time: end_time[i],
                profit: profit[i]
            };
            jobs.push(job);
        }

        jobs.sort_by_key(|x| x.end_time);

        let mut res = vec![0; profit.len()];
        // let mut end_time = vec![0; profit.len()];
        for i in 0..profit.len() { 
            res[i] = jobs[i].profit;
            // end_time[i] = jobs[i].end_time;
            // dbg!(&res);
        }
        
        for i in 1..jobs.len() {
            // println!("Iteration {:?}", i);
            res[i] = cmp::max(res[i-1], res[i]);
            match binary_search(&jobs, i) {
                Some(x) => {
                    res[i] = cmp::max(res[i-1], res[x] + jobs[i].profit);
                    // dbg!(&x, &res);
                },
                None => {}
            }
        }

        let mut largest = res[0];
        for i in 1..res.len() {
            largest = cmp::max(largest, res[i]);
        }
        largest
    }
}

fn binary_search(jobs: &Vec<Job>, space: usize) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = space;

    while low <= high {
        let mid = (low + high)/2;
        if jobs[mid].end_time <= jobs[space].start_time {
            if jobs[mid+1].end_time <= jobs[space].start_time {
                low = mid + 1;
            } else {
                return Some(mid);
            }
        } else {
            high = match mid.checked_sub(1) {
                Some(x) => x,
                None => break 
            };
        }

    }
    return None;
}
