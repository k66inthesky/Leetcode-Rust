use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans:HashMap<i32,i32>=HashMap::new();
        for (i,ele) in nums.iter().enumerate(){
            match ans.get( &(target- *ele) ) {
                Some(&x)=>return vec![i as i32,x],
                None=>ans.insert(*ele, i as i32),
            };
        }
        vec![]
    }
}