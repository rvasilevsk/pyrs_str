use regex::Regex;
use std::borrow::Cow;

// impl str {
//     pub fn simplify(&self) -> Cow<'_, str> {
//         let re_space = Regex::new(r"\s+").unwrap();
//         re_space.replace_all(self.trim(), " ")
//     }
// }

pub fn str_simplify(s: &str) -> Cow<'_, str> {
    let re_space = Regex::new(r"\s+").unwrap();
    re_space.replace_all(s.trim(), " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_simplify() {
        let data = vec![
            ("", ""),
            ("      \r\n\t", ""),
            (" some \t\r\n text  ", "some text"),
        ];
        for (l, r) in data {
            assert_eq!(str_simplify(l), r);
        }
    }
}
