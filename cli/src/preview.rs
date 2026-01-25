use crate::PreviewArgs;
use powerfile_core::interpreter::{AstPrettyPrintInterpreter, Interpreter, TextInterpreter};
use powerfile_core::parser;

pub struct PreviewHandler<'a> {
    pub args: &'a PreviewArgs,
}

impl<'a> PreviewHandler<'a> {
    pub fn handle(&self) {
        let pattern = &self.args.pattern;
        let limit = self.args.limit as usize;

        match parser::parse(pattern) {
            Ok(value) => {
                let start = std::time::Instant::now();

                let result = if self.args.ast {
                    AstPrettyPrintInterpreter.interpret(&value)
                } else {
                    TextInterpreter.interpret(&value)
                };

                let total = result.len();
                let shown = total.min(limit);

                if total > limit {
                    println!("… truncated (showing first {} of {} results)", shown, total)
                }

                for line in result.iter().take(limit) {
                    println!("{}", line)
                }

                if total > limit {
                    println!("--- {} items hidden", total - shown)
                }

                eprintln!("Time elapsed: {:?}", start.elapsed());
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
