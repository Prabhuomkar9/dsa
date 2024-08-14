pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::from("");
        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();

        loop {
            match (chars1.next(), chars2.next()) {
                (Some(ch1), Some(ch2)) => {
                    res.push(ch1);
                    res.push(ch2);
                }
                (Some(ch), None) => res.push(ch),
                (None, Some(ch)) => res.push(ch),
                (None, None) => break,
            }
        }

        res
    }
}
