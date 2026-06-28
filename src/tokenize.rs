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

pub enum Literal {
    String(String),
    Number(f64),
    None,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub tokentype: TokenType,
    pub lexeme: String,
    pub literal : Literal,
    pub line: usize,
}

impl Token {
    pub fn new(tokentype: TokenType, lexeme: impl Into<String>, literal: Literal, line: usize) -> Token {
        Token {
            tokentype,
            lexeme: lexeme.into(),
            literal,
            line,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Tokens { 
    pub tokens: Vec<Token>,
}

#[derive(Debug, PartialEq)]
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

    fn scan_tokens(mut self) -> Result<Tokens, Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::TEof, "", Literal::None, self.line));
        Ok(Tokens { tokens: self.tokens })
    }
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn matches(&mut self, expect:char) -> bool{
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expect {
            return false;
        }
        self.current += 1;
        true
    }

    fn add_token(&mut self, toktype: TokenType) {
        self.add_token_literal(toktype, Literal::None);
    }

    fn add_token_literal(&mut self, toktype: TokenType, literal: Literal) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(toktype, text, literal, self.line));
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenType::TLeftParen),
            ')' => self.add_token(TokenType::TRightParen),
            '{' => self.add_token(TokenType::TLeftBrace),
            '}' => self.add_token(TokenType::TRightBrace),
            ',' => self.add_token(TokenType::TComma),
            '.' => self.add_token(TokenType::TDot),
            '-' => self.add_token(TokenType::TMinus),
            '+' => self.add_token(TokenType::TPlus),
            ';' => self.add_token(TokenType::TSemiColon),
            '*' => self.add_token(TokenType::TStar),
            '!' => {
                let toktype = if self.matches('=') {
                    TokenType::TBangEqual
                } else {
                    TokenType::TBang
                };
                self.add_token(toktype);
            },
            '=' => {
                let toktype = if self.matches('=') {
                    TokenType::TEqualEqual
                } else {
                    TokenType::TEqual
                };
                self.add_token(toktype);
            },
            '<' => {
                let toktype = if self.matches('=') {
                    TokenType::TLessEqual
                } else {
                    TokenType::TLess
                };
                self.add_token(toktype);
            },
            '>' => {
                let toktype = if self.matches('=') {
                    TokenType::TGreaterEqual
                } else {
                    TokenType::TGreater
                };
                self.add_token(toktype);
            },
            _ => todo!()
        }
    }
}

// 💡 串联驱动函数：把读到的 source 真正喂给你的 Scanner
pub fn tokenize(source: Source) -> Result<Tokens, Error> {
    println!("Tokenizing");
    // 创建扫描器并运行
    let scanner = Scanner::new(&source.contents);
    let tokens = scanner.scan_tokens();
    Ok(Tokens {tokens})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive() {
        assert_eq!(true, true);
    }
    #[test]
    fn single_character(){
        let mut source = Scanner::new("(){},.-=;*");
        let tokens = source.scan_tokens();
        assert_eq!(tokens.unwrap().tokens,
            vec![
                Token::new(TokenType::TLeftParen, "(", Literal::None, 1),
                Token::new(TokenType::TRightParen, ")", Literal::None, 1),
                Token::new(TokenType::TLeftBrace, "{", Literal::None, 1),
                Token::new(TokenType::TRightBrace, "}", Literal::None, 1),
                Token::new(TokenType::TComma, ",", Literal::None, 1),
                Token::new(TokenType::TDot, ".", Literal::None, 1),
                Token::new(TokenType::TMinus, "-", Literal::None, 1),
                Token::new(TokenType::TEqual, "=", Literal::None, 1),
                Token::new(TokenType::TSemiColon, ";", Literal::None, 1),
                Token::new(TokenType::TStar, "*", Literal::None, 1),
                Token::new(TokenType::TEof, "", Literal::None, 1)
            ]
        );
    }
    fn tow_characters(){
        let mut scanner = Scanner::new("!= == <= >= ");
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens.unwrap().tokens,
            vec![
                Token::new(TokenType::TBangEqual, "!=", Literal::None, 1),
                Token::new(TokenType::TEqualEqual, "==", Literal::None, 1),
                Token::new(TokenType::TLessEqual, "<=", Literal::None, 1),
                Token::new(TokenType::TGreaterEqual, ">=", Literal::None,   1),
                Token::new(TokenType::TEof, "", Literal::None, 1)
            ]
        );
    }
}