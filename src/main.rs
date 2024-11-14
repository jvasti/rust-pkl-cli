use anyhow::{anyhow, Context, Result};
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand, ValueEnum};
use new_pkl::{Pkl, PklResult};
use std::fs::File;
use std::io::{stdout, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Raw,
    Json,
    Yaml,
    Toml,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Render pkl module(s)
    Eval {
        /// Output format to generate.
        #[arg(short, long, value_enum, default_value_t = Format::Json)]
        format: Format,
        /// File path where the output file is placed.
        #[arg(short = 'o', long = "output_path")]
        path: Option<Utf8PathBuf>,
        /// Module paths or URIs to evaluate.
        #[arg(value_name = "MODULES")]
        modules: Utf8PathBuf,
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

fn read_input_file(path: &Utf8PathBuf) -> Result<String> {
    std::fs::read_to_string(path).context(format!("Failed to read file '{}'", path))
}

fn write_output(path: &Option<Utf8PathBuf>, content: &str) -> Result<()> {
    match path {
        Some(path) => {
            let mut output_file = File::create_new(path)
                .context(format!("Failed to create new output file '{}'", path))?;
            output_file.write_all(content.as_bytes()).context(format!(
                "Failed to write to output file:\nContent: {}",
                content
            ))?;
        }
        None => {
            stdout()
                .write_all(content.as_bytes())
                .context(format!("Failed to write to stdout:\nContent: {}", content))?;
        }
    }

    Ok(())
}

fn parse(input: &str) -> PklResult<Pkl> {
    let mut pkl = Pkl::new();
    pkl.parse(input)?;
    Ok(pkl)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Eval {
            format,
            modules,
            path,
        } => {
            let input = read_input_file(modules)?;
            let parsed_pkl = parse(&input).map_err(|(msg, span)| {
                anyhow!(
                    "{} at {:?}: {:?}",
                    msg,
                    span.clone(),
                    input.get(span).unwrap_or("span not found")
                )
            })?;
            let pkl_variables = &parsed_pkl.table().variables;
            let output = match format {
                Format::Json => serde_json::to_string_pretty(&pkl_variables)
                    .context("Failed to generate JSON.")?,
                Format::Yaml => {
                    serde_yml::to_string(&pkl_variables).context("Failed to generate YAML.")?
                }
                Format::Toml => {
                    toml::to_string_pretty(&pkl_variables).context("Failed to generate TOML")?
                }
                Format::Raw => format!("{:#?}", &pkl_variables),
            };
            write_output(path, &output)?;
        }
        _ => {
            println!("Not implemented");
        }
    }
    Ok(())
}
