use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token<'source> {
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

    // Excludes tokens defined above
    #[regex(r"[^\s\.\,\[\]\(\)]+", |lex| lex.slice())]
    Text(&'source str),
}
