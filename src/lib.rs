//! soundex_rs is a library that calculates the words' soundex.
//!
//! # References
//! <https://support.esri.com/en/technical-article/000003773>
//!
//!  # Features
//! | feature | description  |
//! | --------| -------------|
//! | default | The result retains the first four characters of the soundex value｜
//! | full    | The result retains the complete value of soundex |
//!
//! # Examples
//! ```
//! use soundex_rs::Soundex;
//! println!("{}", "hello world".soundex());
//! ```

use std::ops::Deref;

pub trait Soundex: Deref<Target = str> {
    /// soundex get the string's soundex value.
    /// # Examples
    /// ```
    /// use soundex_rs::Soundex;
    /// if cfg!(feature="full") {
    ///     assert_eq!("hello world".soundex(), "H4643".to_string());
    /// } else {
    ///     assert_eq!("hello world".soundex(), "H464".to_string());
    /// }
    /// ```
    fn soundex(&self) -> String;
}

/// Default implementation for strings.
impl<T: Deref<Target = str>> Soundex for T {
    fn soundex(&self) -> String {
        if self.is_empty() {
            return Default::default();
        }

        let mut r = Vec::with_capacity(4);
        let mut last = None;
        let mut count = 0;

        for next in self.chars() {
            let score = number_map(next);

            if last.is_none() {
                if !next.is_alphanumeric() {
                    continue;
                }

                last = score;
                r.push(next.to_ascii_uppercase());
            } else {
                if !next.is_ascii_alphabetic() || is_drop(next) || score == last {
                    continue;
                }

                last = score;
                r.push(score.unwrap());
            }

            count += 1;

            if !cfg!(feature = "full") && count == 4 {
                break;
            }
        }

        if count < 4 {
            r.extend(vec!['0'; 4 - count])
        }

        r.into_iter().collect()
    }
}

#[inline(always)]
fn number_map(i: char) -> Option<char> {
    match i.to_ascii_lowercase() {
        'b' | 'f' | 'p' | 'v' => Some('1'),
        'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => Some('2'),
        'd' | 't' => Some('3'),
        'l' => Some('4'),
        'm' | 'n' => Some('5'),
        'r' => Some('6'),
        _ => Some('0'),
    }
}

#[inline(always)]
fn is_drop(c: char) -> bool {
    matches!(
        c.to_ascii_lowercase(),
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'h' | 'w'
    )
}

/// equal compares two strings' soundex value, if the result is equal, returns true.
/// # Examples
/// ```
///  use soundex_rs::equal;
///  assert!(equal("Y.LEE", "Y.LIE"));
/// ```
pub fn equal<LEFT, RIGHT>(left: LEFT, right: RIGHT) -> bool
where
    LEFT: Soundex,
    RIGHT: Soundex,
{
    left.soundex() == right.soundex()
}

#[cfg(test)]
mod tests {

    use super::Soundex;
    use crate::equal;

    #[test]
    fn test_soundex() {
        let m = vec![
            ("", "".to_string()),
            ("c你rfpv", "C610".to_string()),
            ("你rfpv", "你610".to_string()),
            ("x", "X000".to_string()),
            ("xxxxx", "X000".to_string()),
            ("difficult", "D1243".to_string()),
            ("Knuth", "K530".to_string()),
            ("Kant", "K530".to_string()),
            ("Jarovski", "J612".to_string()),
            ("Resnik", "R252".to_string()),
            ("Reznick", "R252".to_string()),
            ("Euler", "E460".to_string()),
            ("Peterson", "P3625".to_string()),
            ("Jefferson", "J1625".to_string()),
            ("bb你iiiffpvsgsslkfldsjfasdas", "B24214321232".to_string()),
        ];

        for (i, v) in m.into_iter() {
            if cfg!(feature = "full") {
                assert_eq!(i.soundex(), v, "{}", i);
                assert_eq!(i.to_string().soundex(), v, "{}", i);
                assert_eq!(i.to_string().as_mut().soundex(), v, "{}", i);
            } else {
                assert_eq!(i.soundex(), String::from_iter(v.chars().take(4)), "{}", i);
                assert_eq!(
                    i.to_string().soundex(),
                    String::from_iter(v.chars().take(4)),
                    "{}",
                    i
                );
                assert_eq!(
                    i.to_string().as_mut().soundex(),
                    String::from_iter(v.chars().take(4)),
                    "{}",
                    i
                );
            }
        }
    }

    #[test]
    fn test_equal() {
        assert!(equal("hello", "hello".to_string()));
        assert!(equal("hello", "hello"));
        assert!(!equal("hello world", "hello".to_string()));
    }
}
