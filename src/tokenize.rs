// tokenize.rs

use crate::reader::Source;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // 单字符 Token
    TLeftParen,
    TRightParen,
    TLeftBrace,
    TRightBrace,
    TComma,
    TDot,
    TMinus,
    TPlus,
    TSemiColon,
    TSlash,
    TStar,

    // 一到双字符 Token
    TBang,
    TBangEqual,
    TEqual,
    TEqualEqual,
    TGreater,
    TGreaterEqual,
    TLess,
    TLessEqual,

    // 字面量
    TIdentifier,
    TString,
    TNumber,

    // 关键字
    TAnd,
    TClass,
    TElse,
    TFalse,
    TFun,
    TFor,
    TIf,
    TNil,
    TOr,
    TPrint,
    TReturn,
    TSuper,
    TThis,
    TTrue,
    TVar,
    TWhile,

    TEof,
}

// 允许在当前文件直接使用 TLeftParen 等，不用每次都写 TokenType::
#[allow(unused_imports)]
use TokenType::*;

#[derive(Debug, Clone)]
pub struct Token {
    pub tokentype: TokenType,
    pub lexeme: String,
    pub literal : Literal,
    pub line: usize,
}

impl Token {
    pub fn new(tokentype: TokenType, lexeme: &str, literal: Literal, line: usize) -> Token {
        Token {
            tokentype,
            lexeme: String::from(lexeme),
            literal,
            line,
        }
    }
}

#[derive(Debug)]
pub struct Tokens { 
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct Error {}

struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: &str) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_tokens(mut self) -> Tokens {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::TEof, "", Literal::None, self.line));
        Tokens { tokens: self.tokens }
    }

    fn scan_token(&mut self) {
        todo!()
    }
}

// 💡 串联驱动函数：把读到的 source 真正喂给你的 Scanner
pub fn tokenize(source: Source) -> Result<Tokens, Error> {
    println!("Tokenizing");
    // 创建扫描器并运行
    let scanner = Scanner::new(&source.contents);
    let tokens = scanner.scan_tokens();
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
}