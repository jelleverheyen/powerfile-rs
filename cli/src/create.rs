use powerfile_core::interpreter::{Interpreter, SizeInterpreter, TextInterpreter};
use powerfile_core::parser;
use crate::CreateArgs;

pub struct CreateHandler {
    args: CreateArgs
}

impl CreateHandler {
    pub fn handle(&self) {
        let pattern = &self.args.pattern;

        match parser::parse(pattern) {
            Ok(value) => {
                println!("{:#?}", value);

                let start = std::time::Instant::now();
                let text = TextInterpreter;
                let size = SizeInterpreter.interpret(&value);
                if size > self.args.limit {
                    println!("Pattern size {:#?} exceeds limit of {:#?} ", size, self.args.limit);
                }

                for line in text.interpret(&value) {
                    println!("{}", line)
                }
                eprintln!("{:?}", start.elapsed());
            }
            Err((msg, span)) => {
                use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};

                let mut colors = ColorGenerator::new();
                let a = colors.next();

                Report::build(ReportKind::Error, &pattern, span.end)
                    .with_message("Invalid pattern".to_string())
                    .with_label(Label::new((&pattern, span)).with_message(msg).with_color(a))
                    .finish()
                    .eprint((&pattern, Source::from(&pattern)))
                    .unwrap();
            }
        }
    }
}