#![allow(dead_code)]
/// reference: https://leetcode.cn/problems/reverse-vowels-of-a-string/
pub struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        fn is_vowel(ch: char) -> bool {
            matches!(
                ch,
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'
            )
        }
        let mut vowels = s
            .chars()
            .filter(|&ch| is_vowel(ch))
            .collect::<Vec<_>>()
            .into_iter()
            .rev();

        s.chars()
            .map(|ch| {
                if is_vowel(ch) {
                    vowels.next().unwrap()
                } else {
                    ch
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            "hello".to_string(),
            Solution::reverse_vowels("holle".into())
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            "leotcede".to_string(),
            Solution::reverse_vowels("leetcode".into())
        );
    }
}
