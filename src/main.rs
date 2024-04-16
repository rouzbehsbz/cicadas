use clap::error::Result;
use parser::Parser;

mod http;
mod parser;
mod pool;

//TODO: need to handle errors just like clap
fn main() -> Result<()> {
    let parser = Parser::new();

    let arguments = parser.get_arguments()?;

    println!("{:?}", arguments);

    Ok(())
}
