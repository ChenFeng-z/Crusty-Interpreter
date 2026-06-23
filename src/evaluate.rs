//evaluate.rs
//
//Run a Lox Program

use crate::parser::AST;

pub struct  Output {}
pub type Error = ();

pub fn evaluate(ast : AST) -> Result<Output, Error>{
    println!("Evaluating");
    Ok(Output {  })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive(){
        assert_eq!(true, true);
    }
}