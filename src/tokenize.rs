//tokenize.rs

use std::vec;

use crate::reader::Source;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
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

    // One or two character tokens
    TBang,
    TBangEqual,
    TEqual,
    TEqualEqual,
    TGreater,
    TGreaterEqual,
    TLess,
    TLessEqual,

    // Literals
    TIdentifier,
    TString,
    TNumber,

    // Keywords
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

use TokenType::*;

#[derive(Debug)]
pub struct Token {
    pub tokentype: TokenType,
    pub lexeme: String,
    pub line: usize,
}
#[derive(Debug)]
pub struct  Tokens { 
    pub tokens: Vec<Token>,
}
#[derive(Debug)]
pub struct  Error { }

pub fn tokenize(_source :Source) -> Result<Tokens, Error> {
    println!("Tokenizing");
    let tokens = vec![];
    Ok(Tokens { tokens })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive(){
        assert_eq!(true, true);
    }
}