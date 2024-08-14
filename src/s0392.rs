pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s = s.chars();
        let mut t = t.chars();

        loop {
            match (s.clone().next(), t.next()) {
                (Some(ch1), Some(ch2)) => {
                    if ch1 == ch2 {
                        s.next();
                    }
                }
                (Some(_), None) => return false,
                _ => break,
            }
        }

        true
    }
}
