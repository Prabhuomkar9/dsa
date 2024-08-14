mod s0392;

use s0392::Solution;

fn main() {
    print!(
        "{}",
        Solution::is_subsequence(String::from("ace"), String::from("abcde"))
    )
}
