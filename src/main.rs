use clap::error::Result;
use parser::Parser;

mod parser;

//TODO: need to handle errors just like clap
fn main() -> Result<()> {
    let parser = Parser::new();

    let arguments = parser.get_arguments()?;

    println!("{:?}", arguments);

    Ok(())
}
