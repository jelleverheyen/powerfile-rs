mod create;

use clap::{Args, Parser, Subcommand};
use powerfile_core::interpreter::Interpreter;

#[derive(Parser)]
#[command(name = "PowerFile")]
#[command(version = "0.0.1")]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct PowerFileCli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create files and directories from a pattern
    Create(CreateArgs),
    /// Preview your pattern
    Preview(PreviewArgs),
    /// Manage your template index
    Index(IndexArgs),
}

#[derive(Args)]
struct CreateArgs {
    pattern: String,
    #[arg(default_value_t = 100)]
    limit: u32,
    #[arg(short, long)]
    debug: bool,
    tags: Vec<String>,
}

#[derive(Args)]
struct PreviewArgs {

}

#[derive(Args)]
struct IndexArgs {

}

fn main() {
    //let args = Args::parse();

    // TODO: Implement UNDO/REDO?
    //let pattern = "(Environments/(Dev,Prod)/(Files/(env,settings)[a..z][0..10].json))";
    //let pattern = "[a..z][A..Z][a..z,a..z].cs";
    let pattern = "chinese_studies/chars/[0{a..b}..10]_(我,吃,了,一,个,苹,果).char";
    //let pattern = args.pattern;

    let cli = PowerFileCli::parse();
    let kak = match &cli.command {
        Commands::Create(args) => 1,
        Commands::Preview(args) => 1,
        Commands::Index(args) => 1
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        PowerFileCli::command().debug_assert();
    }
}