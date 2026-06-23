// I'm going to initially break the project into four parts.
// Each in their own file.

mod reader;
mod tokenize;
mod parser;
mod evaluate;

type Error = ();

fn run() -> Result<(), Error> {
    let source = reader::read_source("somefile.lox")?;
    let tokens = tokenize::tokenize(source)?;
    let ast = parser::parse(tokens)?;
    let out = evaluate::evaluate(ast)?;
    Ok(())
}

fn main() {
    println!("Hello, Lox!");
    match run() {
        Ok(_) => {println!("Success!")},
        Err(e) => {println!("Failed {e:?}")},
    }
    
    
}
