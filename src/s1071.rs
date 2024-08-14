pub struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let left = str1.clone() + &str2;
        let right = str2.clone() + &str1;

        if left != right {
            return String::from("");
        }

        let left = str1.len();
        let right = str2.len();
        let gcd_length = Self::gcd(left, right);

        str1[..gcd_length].to_string()
    }

    fn gcd(mut a: usize, mut b: usize) -> usize {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}
