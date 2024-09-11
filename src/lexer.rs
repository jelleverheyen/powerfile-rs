use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'source> {
    #[regex(r"[a-zA-Z0-9_\-\s/\\]+", |lex| lex.slice())]
    Text(&'source str),

    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[token("[")]
    BracketOpen,

    #[token("..")]
    Range,

    #[token(".")]
    Dot,

    #[token("]")]
    BracketClose,

    #[token(",")]
    Comma,
}
