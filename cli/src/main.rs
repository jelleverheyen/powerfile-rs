use clap::Parser;
use lang::interpreter::{Interpreter, SizeInterpreter, TextInterpreter};
use lang::parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pattern: String,
}

fn main() {
    //let args = Args::parse();

    // TODO: Implement UNDO/REDO?
    //let pattern = "(Environments/(Dev,Prod)/(Files/(env,settings)[a..z][0..10].json))";
    //let pattern = "[a..z][A..Z][a..z,a..z].cs";
    let pattern = "chinese_studies/chars/[0{a..b}..10]_(我,吃,了,一,个,苹,果).char";
    //let pattern = args.pattern;

    match parser::parse(&pattern) {
        Ok(value) => {
            println!("{:#?}", value);

            let start = std::time::Instant::now();
            let text = TextInterpreter;
            let size = SizeInterpreter;
            println!("Size: {}", size.interpret(&value));
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
