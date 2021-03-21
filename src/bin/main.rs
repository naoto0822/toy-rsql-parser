use toy_rsql_parser::lexer::Lexer;
use toy_rsql_parser::parser::Parser;

fn main() {
    let query = String::from("SELECT 1+1+2, id, name FROM user WHERE id = 1;");
    println!("query: {}", query);
    let mut lexer = Lexer::new(query);
    //let tokens = lexer.get_tokens();
    //match tokens {
    //    Ok(v) => println!("tokens: {:?}", v),
    //    Err(e) => println!("err: {:?}", e)
    //};

    let mut parser = Parser::new(lexer);
    let stmts = parser.parse();
    println!("ast:\n{:?}", stmts);
}
