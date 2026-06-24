// I'm going to initially break the project into four parts.
// Each in their own file.

mod reader;
mod tokenize;
mod parser;
mod evaluate;

#[derive(Debug)]
enum Error {
    Read(reader::Error),
    Tokenize(tokenize::Error),
    Parse(parser::Error),
    Evaluate(evaluate::Error),
}

impl From<reader::Error> for Error {
    fn from(e: reader::Error) -> Error {
        Error::Read(e)
    }
}

impl From<tokenize::Error> for Error {
    fn from(e: tokenize::Error) -> Error {
        Error::Tokenize(e)
    }
}

impl From<parser::Error> for Error {
    fn from(e: parser::Error) -> Error {
        Error::Parse(e)
    }
}

impl From<evaluate::Error> for Error {
    fn from(e: evaluate::Error) -> Error {
        Error::Evaluate(e)
    }
}

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
