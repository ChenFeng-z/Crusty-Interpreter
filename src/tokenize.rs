//tokenize.rs

use crate::reader::Source;

pub struct  Tokens { }
pub type Error = ();

pub fn tokenize(source :Source) -> Result<Tokens, Error> {
    println!("Tokenizing");
    Ok(Tokens {  })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive(){
        assert_eq!(true, true);
    }
}