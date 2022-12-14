use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
	/// The pattern to look for
	pattern: String,
	/// The path to the file to read
	path: std::path::PathBuf,
}

fn main() -> Result<()> {
	let args = Cli::parse();
	let path = &args.path;

	let content = std::fs::read_to_string(path)
		.with_context(|| format!("could not read file `{}`", path.display()))?;

	grrs::find_matches(&content, &args.pattern, &mut std::io::stdout())?;

	Ok(())
}
