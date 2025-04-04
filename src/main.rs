use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct CodeSnippet {
    pub src: String,
    pub path: Option<PathBuf>,
    pub language: String,
}

struct CodeParser {
    llm_out: Vec<char>,
    cur_idx: usize,
    end_idx: usize,
}

impl CodeParser {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let llm_out_str = fs::read_to_string(path).unwrap();
        let llm_out: Vec<char> = llm_out_str.chars().collect();
        let end_idx = llm_out.len() - 1;
        Self {
            llm_out,
            cur_idx: 0,
            end_idx,
        }
    }

    pub fn from_str(string: &str) -> Self {
        let llm_out: Vec<char> = string.chars().collect();
        let end_idx = llm_out.len() - 1;
        Self {
            llm_out,
            cur_idx: 0,
            end_idx,
        }
    }

    fn peek(&self) -> char {
        self.llm_out[self.cur_idx]
    }

    fn next_char(&mut self) {
        self.cur_idx += 1
    }

    fn skip_char_to(&mut self, num_char_skipped: usize) {
        self.cur_idx += num_char_skipped
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() {
            self.next_char();
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.cur_idx >= self.end_idx - 1
    }

    pub fn parse(&mut self) -> Vec<CodeSnippet> {
        let mut code_snippets = Vec::new();
        let mut code_path = String::new();

        loop {
            while !self.is_at_end() && self.peek() != '`' {
                self.next_char();
            }

            if self.is_at_end() {
                break;
            }

            // skip backticks and check for whitespace. skip every char if whitespace found
            self.skip_char_to(4);
            if !self.peek().is_alphanumeric() {
                while self.peek() != '`' {
                    self.next_char();
                }

                self.cur_idx -= 1;
            } else {
                // if no whitespace go back to parse the programming language
                self.cur_idx -= 1;
            }

            // restart the loop if language is not detected
            if self.peek().is_whitespace() {
                continue;
            }

            // parse the language
            let lang_begin_idx = self.cur_idx;
            while !self.peek().is_whitespace() {
                self.next_char();
            }
            let lang_str: String = self.llm_out[lang_begin_idx..self.cur_idx]
                .into_iter()
                .collect();

            self.skip_whitespace();

            // check if code snippet contains code path and parse it
            if self.peek() == '/' {
                while self.peek() == '/' {
                    self.next_char();
                }
                self.skip_whitespace();
                let path_begin_idx = self.cur_idx;
                while !self.peek().is_whitespace() {
                    self.next_char();
                }

                let path: String = self.llm_out[path_begin_idx..self.cur_idx]
                    .into_iter()
                    .collect();
                code_path.push_str(path.as_str());
            }

            self.skip_whitespace();
            // parse out the code snippet itself
            let src_begin_idx = self.cur_idx;
            while self.peek() != '`' {
                self.next_char();
            }
            let src: String = self.llm_out[src_begin_idx..self.cur_idx]
                .into_iter()
                .collect();
            let code_snippet = CodeSnippet {
                src: src.to_string(),
                path: if code_path.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(code_path.clone()))
                },
                language: lang_str.to_string(),
            };
            code_snippets.push(code_snippet);
            code_path.clear();

            // skip the backticks after everything is parsed
            self.skip_char_to(3);
        }

        code_snippets
    }
}

fn main() {
    let mut snippet_parser = CodeParser::from_path("test_out2.txt");
    let code_snippets = snippet_parser.parse();
    code_snippets
        .iter()
        .map(|snippet| snippet.path.clone().unwrap())
        .for_each(|path| println!("{}", path.display()));

    code_snippets
        .iter()
        .map(|snippet| snippet.language.clone())
        .for_each(|lang| println!("{lang}"));

    code_snippets
        .iter()
        .map(|snippet| snippet.src.clone())
        .for_each(|src| println!("{src}"));
}
