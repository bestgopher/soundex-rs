//! soundex_rs is a library that calculates the words' soundex.
//! Algorithm reference: https://support.esri.com/en/technical-article/000003773
//!
//! | feature | description |
//! | default | The result value retains the first four characters of the soundex value｜
//! | full    | The result retains the complete value of soundex |

#[inline(always)]
fn number_map(i: char) -> Option<char> {
    match i.to_ascii_lowercase() {
        'b' | 'f' | 'p' | 'v' => Some('1'),
        'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => Some('2'),
        'd' | 't' => Some('3'),
        'l' => Some('4'),
        'm' | 'n' => Some('5'),
        'r' => Some('6'),
        _ => None,
    }
}

#[inline(always)]
fn is_drop(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'h' | 'w')
}

/// soundex get the string's soundex value.
/// # Examples
/// ```
/// use soundex_rs::soundex;
/// if cfg!(feature="full") {
///     assert_eq!(soundex("hello world"), "H4643".to_string());
/// } else {
///     assert_eq!(soundex("hello world"), "H464".to_string());
/// }
/// ```
pub fn soundex(s: &str) -> String {
    if s.is_empty() {
        return Default::default();
    }

    let mut r = vec![];
    let mut last = None;

    for next in s.chars() {
        let score = number_map(next);

        if r.is_empty() {
            last = score;
            r.push(next.to_ascii_uppercase());
        } else {
            if is_drop(next) || score.is_none() || score == last {
                continue;
            }

            last = score;
            r.push(score.unwrap());

            if !cfg!(feature = "full") && r.len() == 4 {
                break;
            }
        }
    }

    if r.len() < 4 {
        r.extend(vec!['0'; 4 - r.len()])
    }

    r.into_iter().collect()
}

/// equal compares two strings' soundex value, if the result is equal, returns true.
/// # Examples
/// ```
///  use soundex_rs::equal;
///  assert!(equal("Y.LEE", "Y.LIE"));
/// ```
pub fn equal(left: &str, right: &str) -> bool {
    soundex(left) == soundex(right)
}

#[cfg(test)]
mod tests {
    use super::soundex;

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
                assert_eq!(soundex(i), v, "{}", i);
            } else {
                assert_eq!(soundex(i), String::from_iter(v.chars().take(4)), "{}", i);
            }
        }
    }
}
