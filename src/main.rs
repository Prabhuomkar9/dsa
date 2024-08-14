mod s1071;

use s1071::Solution;

fn main() {
    println!(
        "{}",
        Solution::gcd_of_strings(String::from("ABCABCABC"), String::from("ABC"))
    );
}
