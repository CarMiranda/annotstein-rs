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
		flatten: bool,
	},
}

fn main() {
    let cli = Cli::parse();
	match &cli.command {
		Commands::Validate => {
			let ds = coco::models::Dataset::parse_file(cli.infile).unwrap();
			let _ = ds.validate();
		},
		Commands::Rebase { root_path, flatten } => {
			let mut ds = coco::models::Dataset::parse_file(cli.infile).unwrap();
			for image in ds.images.iter_mut() {
				image.file_name = root_path.to_str().unwrap().to_string();
			}
		},
	}
}
