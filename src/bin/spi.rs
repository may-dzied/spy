use spy::{lexer, parser, runtime};

fn main() {

    let text = "print (+ (+ 2 3) (+ 4 5))";

    let tokens = lexer::lex(text.to_string());
    println!("{:#?}", tokens);

    let ast = parser::parse(tokens);
    println!("{:#?}", ast);

    runtime::execute_ast(ast);

}
