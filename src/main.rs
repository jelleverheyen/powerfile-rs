use crate::visitor::{TextInterpreter, Visitor};

mod parser;
mod lexer;
mod visitor;

fn main() {
    let pattern = "Environments/(Dev,Prod/(Features,Tests,Services))/test.json";

    match parser::parse(pattern) {
        Ok(value) => {
            println!("OK: {:#?}", value);

            let mut interpreter = TextInterpreter;
            for line in interpreter.visit_value(value) {
                println!("{}", line)
            }
        },
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
