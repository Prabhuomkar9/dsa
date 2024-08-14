mod s0283;

use s0283::Solution;

fn main() {
    let mut nums = vec![1, 0];
    Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);
}
