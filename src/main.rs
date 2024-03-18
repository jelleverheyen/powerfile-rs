use logos::{Lexer, Logos, Span};

type Error = (String, Span);
type Result<T> = std::result::Result<T, Error>;

#[derive(Logos, Debug, PartialEq)]
enum Token<'source> {
    #[regex(r"[a-zA-Z0-9_\-\.\s/\\]+", |lex| lex.slice())]
    String(&'source str),

    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[token(",")]
    Comma,
}

#[derive(Debug)]
enum Value<'source> {
    ExpandableGroup(Vec<Value<'source>>),
    TextGroup(Vec<Value<'source>>),
    String(&'source str),
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
            Ok(Token::String(s)) => current_group.push(Value::String(s)),
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

fn main() {
    let pattern = "Environments/(Dev,Prod/(Features,Tests,Services))/test.json";

    let mut lexer = Token::lexer(pattern);

    match parse_group(&mut lexer, false) {
        Ok(value) => println!("OK: {:#?}", value),
        Err((msg, span)) => {
            use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};

            let mut colors = ColorGenerator::new();
            let a = colors.next();

            Report::build(ReportKind::Error, &pattern, span.end)
                .with_message("Invalid pattern".to_string())
                .with_label(Label::new((&pattern, span)).with_message(msg).with_color(a))
                .finish()
                .eprint((&pattern, Source::from(pattern)))
                .unwrap();
        }
    }
}
