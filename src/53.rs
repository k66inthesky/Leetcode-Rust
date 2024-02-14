impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (mut lmax,mut gmax) = (0,nums[0]);
        for &n in nums.iter(){
            lmax=n.max(lmax+n);
            gmax=gmax.max(lmax);
        }
        gmax
    }
}