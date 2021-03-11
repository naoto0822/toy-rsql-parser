use toy_rsql_parser::lexer::Lexer;
use toy_rsql_parser::token::Token;
use toy_rsql_parser::token::TokenType;

fn main() {
    println!("Start Lexer!");

    let query = String::from("SELECT * FROM user WHERE;");
    let mut lexer = Lexer::new(query);
    let mut current_token = Token::start_token();
    while current_token.token_type != TokenType::EOF {
        current_token = lexer.next_token();
        println!("{:?}", current_token);
    }

    println!("End Lexer!");

}
