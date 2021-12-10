use regex::Regex;
use std::fmt::Error;

lazy_static! {
    static ref CHINESE_REGEX: Regex = Regex::new(r"[\u4e00-\u9fa5]").unwrap();
    static ref SPLIT_CHAR: Vec<char> = vec![' ', ','];
}

pub fn read_file(file_path: &str) -> Result<&str, Error> {
    unimplemented!("111")
}

fn write_file(file_path: &str) -> Result<&str, Error> {
    unimplemented!()
}

fn is_chinese_letter(input: &char) -> bool {
    CHINESE_REGEX.is_match(&input.to_string())
}

fn is_english_letter(input: &char) -> bool {
    input.is_ascii_alphabetic()
}

fn is_split_char(input: &char) -> bool {
    SPLIT_CHAR.iter().find(|&x| input == x).is_some()
}

// fn is_dup_char(vec: Vec<String>, )

fn read_string(input: String) -> Vec<String> {
    let mut ret: Vec<String> = Vec::with_capacity(30);

    // store for
    let mut english_store: String = String::new();

    input.chars().for_each(|c| {
        // check is split charactor and english_store is not empty
        if is_split_char(&c) && !english_store.is_empty() {
            ret.push(english_store.clone());
            english_store.clear();
            return;
        }

        // check is chinese charactor, add to return
        if is_chinese_letter(&c) {
            if !english_store.is_empty() {
                ret.push(english_store.clone());
                english_store.clear();
            }
            ret.push(c.to_string());
            return;
        }

        // check is english charactor, add to english_store
        if is_english_letter(&c) {
            english_store.push(c);
            return;
        }
    });

    if !english_store.is_empty() {
        ret.push(english_store.clone());
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_chinese_letter() {
        assert!(is_chinese_letter(&'哈'));
        assert!(!is_chinese_letter(&'a'));
        assert!(!is_chinese_letter(&'b'));
    }
    #[test]
    fn test_is_english_letter() {
        assert!(is_english_letter(&'a'));
        assert!(is_english_letter(&'b'));
        assert!(is_english_letter(&'z'));
        assert!(!is_english_letter(&','));
        assert!(!is_english_letter(&'!'));
        assert!(!is_english_letter(&'哈'));
    }

    #[test]
    fn test_is_split_char() {
        assert!(!is_split_char(&'c'));
        assert!(!is_split_char(&'f'));
        assert!(is_split_char(&' '));
        assert!(is_split_char(&','));
    }
    #[test]
    fn test_read_string() {
        let input = String::from("apple,banana");
        assert_eq!(read_string(input), vec!["apple", "banana"]);

        let input = String::from("我是谁");
        assert_eq!(read_string(input), vec!["我", "是", "谁"]);

        let input = String::from("apple,banana我是谁");
        assert_eq!(
            read_string(input),
            vec!["apple", "banana", "我", "是", "谁"]
        );
    }
}
