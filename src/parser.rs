use logos::{Lexer, Logos, Span};
use crate::lexer::Token;

type Error = (String, Span);
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Value<'source> {
    ExpandableGroup(Vec<Value<'source>>),
    TextGroup(Vec<Value<'source>>),
    Text(&'source str),
}

pub fn parse(
    pattern: &str,
) -> Result<Value> {
    let mut lexer = Token::lexer(pattern);

    parse_group(&mut lexer, false)
}

fn parse_group<'source>(
    lexer: &mut Lexer<'source, Token<'source>>,
    explicit_close: bool,
) -> Result<Value<'source>> {
    // Used to build the text group
    let mut children = Vec::new();
    // Used to build the expandable group
    let mut current_group = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::Text(s)) => current_group.push(Value::Text(s)),
            Ok(Token::Comma) => {
                if !children.is_empty() {
                    children.push(Value::ExpandableGroup(std::mem::take(&mut current_group)));
                }
            }
            Ok(Token::ParenOpen) => current_group.push(parse_group(lexer, true)?),
            Ok(Token::ParenClose) if explicit_close => {
                if !current_group.is_empty() {
                    children.push(Value::ExpandableGroup(current_group))
                }

                return Ok(Value::TextGroup(children));
            }
            Ok(Token::ParenClose) => {
                return Err(("Unexpected group closer ')'".to_owned(), lexer.span()))
            }
            _ => return Err(("Unexpected token".to_owned(), lexer.span())),
        }
    }

    if explicit_close {
        return Err(("Expected ')' before end of input".to_owned(), lexer.span()));
    }

    if !current_group.is_empty() {
        children.push(Value::ExpandableGroup(current_group))
    }

    Ok(Value::TextGroup(children))
}