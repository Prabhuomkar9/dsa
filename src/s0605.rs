pub struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut vowels = vec![];

        for c in s.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => vowels.push(c),
                _ => {}
            }
        }

        let mut res = String::from("");
        vowels.reverse();
        let mut vowels = vowels.iter();

        for c in s.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => {
                    if let Some(&ch) = vowels.next() {
                        res.push(ch);
                    }
                }
                _ => res.push(c),
            }
        }

        res
    }
}
