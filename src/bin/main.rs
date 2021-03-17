use toy_rsql_parser::lexer::Lexer;
use toy_rsql_parser::parser::Parser;

fn main() {
    let query = String::from("SELECT id, name FROM user;");
    println!("query: {}", query);
    let lexer = Lexer::new(query);
    let mut parser = Parser::new(lexer);
    let stmts = parser.parse();
    println!("ast:\n{:?}", stmts);
}
