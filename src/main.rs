use clap::Parser;
use std::path::PathBuf;

macro_rules! subcommands {
    ( $( $name:ident : $module:ident ),+ ) => {
        #[derive(clap::Subcommand)]
        enum Subcommands {
            $(
                $name(<$module::ProblemDesc as challenges_input::ProblemDesc>::Args),
            )+
        }
        impl Subcommands {
            fn run(&self) -> String {
                match self {
                    $(
                        Subcommands::$name(args) => $module::run(args),
                    )+
                }
            }
        }
    };
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Subcommands,
    #[arg(short, long, value_name = "PATH")]
    output_path: Option<PathBuf>,
    /// Copy output to clipboard
    #[arg(long)]
    copy: bool,
}

subcommands!(Rowan2025: rowan2025, Aoc: aoc);

fn main() {
    let args = Args::parse();

    let output = args.command.run();

    if args.copy {
        if let Err(e) = copy(&output) {
            eprintln!("Failed to copy to clipboard: {e}");
        } else {
            println!("Output copied to clipboard. Output:\n");
        }
    }

    match args.output_path {
        Some(path) => {
            std::fs::write(path, output).expect("Failed to write output file.");
        }
        None => println!("{output}"),
    }
}

fn copy(text: &str) -> Result<(), arboard::Error> {
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(text)?;
    Ok(())
}
