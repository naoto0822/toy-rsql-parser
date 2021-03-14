use toy_rsql_parser::lexer::Lexer;

fn main() {
    println!("Start Lexer!");

    let query = String::from("SELECT * FROM user WHERE;");
    let mut lexer = Lexer::new(query);
    let tokens = lexer.get_tokens();
    println!("{:?}", tokens);

    println!("End Lexer!");
}
