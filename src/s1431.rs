pub struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut greatest = 0;

        for candy in candies.iter().cloned() {
            if candy > greatest {
                greatest = candy;
            }
        }

        let mut res = vec![];

        for candy in candies.iter().cloned() {
            if candy + extra_candies >= greatest {
                res.push(true)
            } else {
                res.push(false)
            }
        }

        res
    }
}
