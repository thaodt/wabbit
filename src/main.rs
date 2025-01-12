use clap::{Parser, Subcommand};
use std::io::Read;
use std::path::PathBuf;
use twabbit::DisplayToken;

use twabbit::input::Input;
use twabbit::lexer::Lexer;

/// Command line interface of the twabbit compiler.
#[derive(clap::Parser)]
#[command(
    author,
    version,
    about,
    long_about = "A compiler for the Wabbit language."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Tokenize the input and display the tokens in the standard output.
    #[clap(aliases = &["lexer"])]
    Tokenize {
        /// path to the Wabbit source file if any.
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// the Wabbit src code as a string (overrides the path).
        #[arg(short)]
        code: Option<String>,
    },

    /// Parse the input and display the AST in the terminal.
    #[clap(aliases = &["parser"])]
    Parse {
        /// path to the Wabbit source file if any.
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// the Wabbit src code as a string (overrides the path).
        #[arg(short)]
        code: Option<String>,
    },

    /// Interpret the wabbit program.
    Interp {
        /// path to the Wabbit source file if any.
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// the Wabbit src code as a string (overrides the path).
        #[arg(short)]
        code: Option<String>,
    },

    /// Run our formatter on the code.
    #[clap(aliases = &["fmt"])]
    Format {
        /// path to the Wabbit source file if any.
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// the Wabbit src code as a string (overrides the path).
        #[arg(short)]
        code: Option<String>,
    },
}

/// Get the source code from the command line arguments.
fn get_source(path: Option<PathBuf>, code: Option<String>) -> anyhow::Result<String> {
    if let Some(code) = code {
        Ok(code)
    } else if let Some(path) = path {
        Ok(std::fs::read_to_string(path)?)
    } else {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}

/// Entry point of the program.
fn main() -> anyhow::Result<()> {
    // parse the command line arguments.
    let cli = Cli::parse();
    // execute the command.
    match cli.command {
        Commands::Tokenize { path, code } => {
            let source = get_source(path, code)?;
            let tokens = Lexer::tokenize(&Input::new(&source))?;
            for token in tokens {
                println!("{}", DisplayToken(token));
            }
        }
        Commands::Parse { path: _, code: _ } => todo!(),
        Commands::Interp { path: _, code: _ } => todo!(),
        Commands::Format { path: _, code: _ } => todo!(),
    }

    Ok(())
}
