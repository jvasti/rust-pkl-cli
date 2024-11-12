use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Text,
    Json,
    Yaml,
    Toml,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Render pkl module(s)
    Eval {
        /// Output format to generate.
        #[arg(short, long, value_enum, default_value_t = Format::Text)]
        format: Format,
        /// File path where the output file is placed.
        #[arg(short = 'o', long = "output_path")]
        path: Option<String>,
        /// Module paths or URIs to evaluate.
        #[arg(value_name = "MODULES", default_value = "stdin")]
        modules: Vec<String>,
    },
    /// (not implemented) Start a REPL session
    Repl {},
    /// (not implemented) Run as a server that communicates over standard input/output
    Server {},
    /// (not implemented) Run tests within the given module(s)
    Test {},
    /// (not implemented) Run commands related to projects
    Project {},
    /// (not implemented) Download package(s)
    DownloadPackage {},
    /// (not implemented) Commands related to static analysis
    Analyze {},
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Eval {
            format,
            modules,
            path,
        } => {
            println!(
                " modules: {:?}\n format:{:?}\n path:{:?}",
                modules, format, path
            )
        }
        _ => println!("Not implemented"),
    }
}
