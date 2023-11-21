use annotstein::coco;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to a COCO annotations file
    #[arg(short, long, value_name = "infile")]
    infile: PathBuf,

    /// Path to a COCO annotations file
    #[arg(short, long, value_name = "outfile")]
    outfile: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validates the input file
    Validate,

	/// Rebase image filename
	Rebase {
		root_path: PathBuf,
	},
}

fn main() {
    let cli = Cli::parse();
	match &cli.command {
		Commands::Validate => {
			match coco::models::Dataset::parse_file(cli.infile) {
                Err(e) => println!("Error parsing file: {}", e),
                Ok(ds) => {
                    let _ = ds.validate();
                }
            }
		},
		Commands::Rebase { root_path } => {
			match coco::models::Dataset::parse_file(cli.infile) {
                Err(e) => println!("Error parsing file: {}", e),
                Ok(mut ds) => {
                    ds.rebase(root_path);
                    if let Some(output_path) = &cli.outfile {
                        let _ = ds.dump_file(output_path);
                    }
                }
            }
		},
	}

}
