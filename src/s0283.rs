pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..nums.len() {
            while j < nums.len() && nums[j] == 0 {
                j += 1;
            }

            if j < nums.len() {
                nums.swap(i, j);
                j += 1;
            }
        }
    }
}
