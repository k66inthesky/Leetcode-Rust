impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![1;n];
        // Forward Product
        for i in 1..n{
            ans[i] = nums[i-1]*ans[i-1]
        }

        // Backward Product
        let mut r = 1;
        for i in (0..n).rev(){
            ans[i]=ans[i]*r;
            r = r * nums[i];
        }

        ans
    }
}