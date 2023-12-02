pub mod token;
mod lexer;
mod repl;
mod ast;
mod parser;

fn main() {
    println!("Hello, this is the Monkey programming language!");

    repl::start(std::io::stdin(), std::io::stdout());
}
