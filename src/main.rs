mod s0088;

use s0088::Solution;

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![4, 5, 6];

    Solution::merge(&mut nums1, 3, &mut nums2, 3);

    print!("{:?}", nums1);
}
