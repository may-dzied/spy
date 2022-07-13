#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Integer(i64),
    OpenBracket,
    CloseBracket
}

pub fn generate_token(token_builder: &mut String) -> Option<Token> {
    let result;
    if *token_builder == String::new() {
        result = None;
    } else if token_builder.chars().all(|x| x.is_ascii_digit()) {
        result = Some(Token::Integer((*token_builder).parse::<i64>().unwrap()));
    } else {
        result = Some(Token::Identifier((*token_builder).clone()));
    }
    *token_builder = String::new();
    result
}

pub fn maybe_add_token(token_builder: &mut String, result: &mut Vec<Token>) {
    let token = generate_token(token_builder);
    if let Some(inner) = token {
        result.push(inner);
    }
}
pub fn lex(text: String) -> Vec<Token> {

    let mut result = vec![];
    let mut token_builder = String::new();

    for chr in text.chars() {
        if chr == '(' {
            maybe_add_token(&mut token_builder, &mut result);
            result.push(Token::OpenBracket);
        } else if chr == ')' {
            maybe_add_token(&mut token_builder, &mut result);
            result.push(Token::CloseBracket);
        } else if chr == ' ' {
            maybe_add_token(&mut token_builder, &mut result);
        } else {
            token_builder.push(chr);
        }
    }

    result
}
