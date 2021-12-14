use regex::Regex;
use std::{collections::HashSet, fmt::Error, fs};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

lazy_static! {
    static ref CHINESE_REGEX: Regex = Regex::new(r"[\u4e00-\u9fa5]").unwrap();
    static ref SPLIT_CHAR: Vec<char> = vec![' ', ',', '\n'];
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

fn is_dup_char(input: &char, got_dict: &Vec<&str>) -> bool {
    got_dict
        .iter()
        .find(|x| x.to_string() == input.to_string())
        .is_some()
}

pub fn read_string(input: &str) -> HashSet<String> {
    let mut ret: HashSet<String> = HashSet::new();

    // store for
    let mut store: String = String::new();

    input.chars().for_each(|c| {
        // check is split charactor and english_store is not empty
        if is_split_char(&c) && !store.is_empty() {
            ret.insert(store.clone());
            store.clear();
            return;
        }

        // // check is chinese charactor, add to return
        // if is_chinese_letter(&c) {
        //     if !store.is_empty() {
        //         ret.push(store.clone());
        //         store.clear();
        //     }
        //     store.push(c);
        //     return;
        // }

        // check is english charactor, add to english_store
        if is_english_letter(&c) {
            store.push(c);
            return;
        }
    });

    if !store.is_empty() {
        ret.insert(store.clone());
    }
    ret
}

pub(crate) fn read_file_2_set(file_path: &str) -> HashSet<String> {
    let mut ret: HashSet<String> = HashSet::new();
    let input = File::open(file_path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        for x in &read_string(&line.unwrap()) {
            ret.insert(x.clone());
        }
    }

    ret
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // fn test_is_chinese_letter() {
    //     assert!(is_chinese_letter(&'哈'));
    //     assert!(!is_chinese_letter(&'a'));
    //     assert!(!is_chinese_letter(&'b'));
    // }
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
        let mut result: HashSet<String> = HashSet::new();
        result.insert(String::from("apple"));
        result.insert(String::from("banana"));

        assert_eq!(read_string(&input), result);

        let input = String::from("我是谁");
        let mut result: HashSet<String> = HashSet::new();
        assert_eq!(read_string(&input), result);

        let input = String::from("apple,banana我是谁");
        let mut result: HashSet<String> = HashSet::new();
        result.insert(String::from("apple"));
        result.insert(String::from("banana"));
        assert_eq!(read_string(&input), result);
    }

    #[test]
    fn test_is_dup_char() {
        let got_dict = vec!["1", "2", "3"];
        assert!(is_dup_char(&'1', &got_dict));
        assert!(is_dup_char(&'2', &got_dict));
        assert!(is_dup_char(&'3', &got_dict));
        assert!(!is_dup_char(&'4', &got_dict));
    }
}

pub fn write_set_2_file(file_path: &str, difference: HashSet<&String>) {
    if let Err(e) = fs::remove_file(&file_path) {
        println!("{:?}", e);
    }

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path).unwrap();

    for c in difference {
        print!("{} ", c);
        output_file.write_all(format!("{}{}", c, "\n").as_bytes()).unwrap();
    }
}