impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut min_so_far, mut max_so_far, mut result)=(1,1,nums[0]);
        for &n in nums.iter().skip(0){
            let (nmin,nmax) = (n*min_so_far, n*max_so_far);
            // min_so_far=*[n,nmin,nmax].iter().min().unwrap();
            min_so_far=n.min(nmin).min(nmax);
            max_so_far=*[n,nmin,nmax].iter().max().unwrap();
            max_so_far=n.max(nmin).max(nmax);
            // println!("{} {}",min_so_far,max_so_far);
            result=result.max(max_so_far)
        }
        result
    }
}