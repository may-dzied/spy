use crate::lexer::Token;

#[derive(Debug)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64)
}

#[derive(Debug)]
pub enum AstNode {
    Group(Vec<AstNode>),
    Identifier(String),
    Value(Value),
    None
}

pub fn parse(tokens: Vec<Token>) -> AstNode {

    let mut result = vec![];
    let mut group_builder = vec![];
    let mut group_nested_level = 0;

    for token in tokens {

        match token {
            Token::OpenBracket => {
                group_nested_level += 1;
                if group_nested_level > 1 {
                    group_builder.push(token);
                }
            },
            Token::CloseBracket => {
                group_nested_level -= 1;
                if group_nested_level == 0 {
                    result.push(parse(group_builder));
                    group_builder = vec![];
                } else {
                    group_builder.push(token);
                }
            },
            Token::Identifier(ref ident) => {
                if group_nested_level == 0 {
                    result.push(AstNode::Identifier(ident.to_string()));
                } else {
                    group_builder.push(token);
                }
            },
            Token::Integer(value) => {
                if group_nested_level == 0 {
                    result.push(AstNode::Value(Value::Integer(value)));
                } else {
                    group_builder.push(token);
                }
            }
        }

    }

    AstNode::Group(result)

}
