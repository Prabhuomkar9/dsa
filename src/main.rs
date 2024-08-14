mod s1768;

use s1768::Solution;

fn main() {
    println!(
        "{}",
        Solution::merge_alternately("abc".to_string(), "pqrst".to_string())
    );
}
