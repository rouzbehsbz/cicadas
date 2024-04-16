use parser::Parser;

mod parser;

fn main() {
    let parser = Parser::new();
    //TODO: need to handle errors just like clap
    let arguments = parser.get_arguments().map_err(|err| err.render());

    println!("{:?}", arguments)
}
