impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len()==1 {return nums[0];}
        let (mut l, mut r)=(0,nums.len()-1);
        if nums[r]>nums[0]{return nums[0];}
        while true{
            let mut mid = l+(r-l)/2;
            
            // Return value condition: Check if mid is the pivot point
            if nums[mid]>nums[mid+1]{return nums[mid+1];}
            if nums[mid-1]>nums[mid]{return nums[mid];}

            // Swap condition: Decide which half to search based on nums[0]
            if nums[mid]>nums[0]{l=mid+1;}
            else{r=mid-1;}

        }
        // Must! Or will error!
        unreachable!();
    }
}