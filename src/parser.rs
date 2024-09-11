use crate::lexer::Token;
use crate::parser::Value::{CharRange, ExpandableGroup, NumberRange};
use logos::{Lexer, Logos, Span};
use std::mem::take;

type Error = (String, Span);
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Value<'source> {
    ExpandableGroup(Vec<Value<'source>>),
    TextGroup(Vec<Value<'source>>),
    Text(&'source str),
    CharRange(char, char),
    NumberRange(u32, u32),
}

pub fn parse(pattern: &str) -> Result<Value> {
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
            Ok(Token::Dot) => current_group.push(Value::Text(".")),
            Ok(Token::Range) => current_group.push(Value::Text("..")),
            Ok(Token::Comma) => {
                if !current_group.is_empty() {
                    children.push(ExpandableGroup(take(&mut current_group)));
                }
            }
            Ok(Token::ParenOpen) => current_group.push(parse_group(lexer, true)?),
            Ok(Token::ParenClose) if explicit_close => {
                if !current_group.is_empty() {
                    children.push(ExpandableGroup(current_group))
                }

                return Ok(Value::TextGroup(children));
            }
            Ok(Token::ParenClose) => {
                return Err(("Unexpected group closer ')'".to_owned(), lexer.span()))
            }
            Ok(Token::BracketOpen) => {
                current_group.push(parse_range(lexer)?);
            }
            _ => return Err(("Unexpected token".to_owned(), lexer.span())),
        }
    }

    if explicit_close {
        return Err(("Expected ')' before end of input".to_owned(), lexer.span()));
    }

    if !current_group.is_empty() {
        children.push(ExpandableGroup(current_group))
    }

    Ok(Value::TextGroup(children))
}

// Handle ranges
#[derive(Debug)]
enum RangeMember<'source> {
    String(&'source str),
    Number(u32),
}

fn parse_range<'source>(lexer: &mut Lexer<'source, Token<'source>>) -> Result<Value<'source>> {
    let mut passed_range_operator = false;

    let mut start = None;
    let mut end = None;

    let mut children = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(Token::BracketOpen) => children.push(parse_range(lexer)?),
            Ok(Token::Text(s)) => {
                let member = match s.parse::<u32>() {
                    Ok(num) => RangeMember::Number(num),
                    Err(_) => RangeMember::String(s),
                };

                match (start.as_mut(), end.as_mut()) {
                    (None, None) => start = Some(member),
                    (Some(_), None) => end = Some(member),
                    _ => return Err(("Failed to parse range expression".to_owned(), lexer.span())),
                }
            }
            Ok(Token::Range) => {
                passed_range_operator = true;
                match (start.as_ref(), end.as_ref()) {
                    (Some(_), None) => {}
                    _ => return Err(("Invalid range operator found".to_owned(), lexer.span())),
                }
            }
            Ok(Token::Comma) if passed_range_operator => {
                passed_range_operator = false;
                children.push(new_range(lexer, start.take(), end.take())?)
            }
            Ok(Token::BracketClose) if passed_range_operator => {
                children.push(new_range(lexer, start.take(), end.take())?);

                return Ok(ExpandableGroup(children));
            }
            Ok(Token::Comma) | Ok(Token::BracketClose) if !passed_range_operator => {
                return Err((
                    "Expected range operator '..', received range close instead".to_owned(),
                    lexer.span(),
                ))
            }
            _ => {
                return Err((
                    "Unexpected token when parsing range".to_owned(),
                    lexer.span(),
                ))
            }
        }
    }

    Err(("Expected ']' before end of input".to_owned(), lexer.span()))
}

fn new_range<'source>(
    lexer: &Lexer<'source, Token<'source>>,
    start: Option<RangeMember<'source>>,
    end: Option<RangeMember<'source>>,
) -> Result<Value<'source>> {
    match (start, end) {
        (Some(RangeMember::Number(s)), Some(RangeMember::Number(e))) => Ok(NumberRange(s, e)),
        (Some(RangeMember::String(s)), Some(RangeMember::String(e))) => {
            match (s.parse::<char>(), e.parse::<char>()) {
                (Ok(s), Ok(e)) => Ok(CharRange(s, e)),
                (_, _) => Err((
                    "Found one or more invalid range members".to_owned(),
                    lexer.span(),
                )),
            }
        }
        _ => Err((
            "Unexpected token when parsing range".to_owned(),
            lexer.span(),
        )),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_valid_range_expression() {
//         let tokens = vec![
//             Token::Text("start"),
//             Token::RangeOperator,
//             Token::Text("end"),
//             Token::BracketClose,
//         ];
//         let mut lexer = Lexer::new(&tokens);
//         let result = parse_range(&mut lexer);
//         assert_eq!(result, Ok(Value::RangeExpression(Value::Text("start"), Value::Text("end"))));
//     }
// }
