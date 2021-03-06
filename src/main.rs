mod lexer;

fn main() {
    println!("Start Lexer!");

    let query = String::from("SELECT * FROM hoge");
    let mut lexer = lexer::Lexer::new(query);
    let mut current_token = lexer::Token::start_token();
    while current_token.token_type != lexer::TokenType::EOF {
        current_token = lexer.next_token();
        println!("{:?}", current_token);
    }

    println!("End Lexer!");

}
