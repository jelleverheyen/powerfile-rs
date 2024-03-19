use crate::interpreter::{Interpreter, TextInterpreter};

mod lexer;
mod parser;
mod interpreter;

fn main() {
    let pattern = "(Environments/(Dev,Prod)/(Files/(env,settings).json))";

    match parser::parse(pattern) {
        Ok(value) => {
            println!("OK: {:#?}", value);

            let mut interpreter = TextInterpreter;
            for line in interpreter.interpret(&value) {
                println!("{}", line)
            }
        }
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
