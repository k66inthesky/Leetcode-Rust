impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        
        let mut p1 = m - 1;
        let mut p2 = n - 1;

        for i in (0..(m + n)).rev() {
            let i = i as usize;

            if p2 < 0 {
                break;
            }

            if p1 >= 0 && nums1[p1 as usize] > nums2[p2 as usize] {
                nums1[i] = nums1[p1 as usize];
                p1 -= 1;
            } 

            else {
                nums1[i] = nums2[p2 as usize];
                p2 -= 1;
            }
        }
    }
}