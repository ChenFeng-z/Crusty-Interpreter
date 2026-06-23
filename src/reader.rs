//reader.rs
//
// Read source code from a file

pub type Source = ();

pub fn read_source(filename: &str) -> Source{
    println!("Reading source");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_alive(){
        assert_eq!(true, true);
    }
}