pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
            return;
        }

        let mut i = m - 1;
        let mut j = n - 1;
        let mut k = m + n - 1;

        loop {
            if k < 0 {
                return;
            }
            let di = i as usize;
            let dj = j as usize;
            let dk = k as usize;
            if i >= 0 && j >= 0 {
                if nums1[di] > nums2[dj] {
                    nums1[dk] = nums1[di];
                    i -= 1;
                } else {
                    nums1[dk] = nums2[dj];
                    j -= 1;
                }
            } else if j >= 0 {
                nums1[dk] = nums2[dj];
                j -= 1;
            }
            k -= 1;
        }
    }
}
