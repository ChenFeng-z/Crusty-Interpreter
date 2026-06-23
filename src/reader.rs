//reader.rs
//
// Read source code from a file

pub struct Source { }
pub type  Error = ();

pub fn read_source(filename: &str) -> Result<Source, Error>{
    println!("Reading source");
    Ok(Source {  })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive(){
        assert_eq!(true, true);
    }
}