use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

struct SourceCode {
    pub src: String,
    pub path: Option<PathBuf>,
    pub language: String,
}

fn main() {
    let mut llm_file = File::open("test_out1.txt").unwrap();
    let mut src_code_str = String::new();
    llm_file.read_to_string(&mut src_code_str).unwrap();
    let mut cur_idx = 0;
    let end_idx = src_code_str.chars().count() - 1;
    loop {
        // look for code
        while cur_idx < end_idx && src_code_str.chars().nth(cur_idx).unwrap() != '`' {
            cur_idx += 1;
        }
        if cur_idx >= end_idx {
            break;
        }
        // parse the language
        cur_idx += 3;
        let lang_first_letter_idx = cur_idx;
        while !src_code_str.chars().nth(cur_idx).unwrap().is_whitespace() {
            cur_idx += 1;
        }
        let lang_str = &src_code_str[lang_first_letter_idx..cur_idx];
        while src_code_str.chars().nth(cur_idx).unwrap().is_whitespace() {
            cur_idx += 1;
        }
        let src_code_first_idx = cur_idx;
        while src_code_str.chars().nth(cur_idx).unwrap() != '`' {
            cur_idx += 1;
        }
        let final_src = &src_code_str[src_code_first_idx..cur_idx];
        // skip the backticks
        cur_idx += 3;

        while !src_code_str.chars().nth(cur_idx).unwrap().is_whitespace() {
            cur_idx += 1;
        }
    }
}
