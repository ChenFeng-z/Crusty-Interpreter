// I'm going to initially break the project into four parts.
// Each in their own file.

use crate::reader::Source;

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

fn run (source: reader::Source) -> Result<(), Error> {
    let tokens = tokenize::tokenize(source)?;
    let ast = parser::parse(tokens)?;
    let out = evaluate::evaluate(ast)?;
    Ok(())
}

fn run_file(filename: &str) -> Result<(), Error> {
    let source = reader::read_source(filename)?;
    run(source)
}

fn run_prompt(){
    todo!()
}

fn main() {
    println!("Hello, Lox!");

    let args : Vec<String> = std::env::args().collect();
    if args.len() == 1{
        run_prompt();
    } else if args.len() == 2{
        match run_file(&args[1]) {
            Ok(_) => {println!("Success!")},
            Err(e) => {eprintln!("Failed! {e:?}")},
        }
    } else {
        eprintln!("Usage: lox [filename]");
    }
    
    
    
}
