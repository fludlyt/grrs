use clap::Parser;

/// Search fpt a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
	/// The pattern to look for
	pattern: String,
	/// The path to the file to read
	path: std::path::PathBuf,
}

fn main() {
	let args = Cli::parse();

	let result = std::fs::read_to_string(&args.path);
	let content = match result {
		Ok(content) => content,
		Err(error) => {
			panic!("Can't deal with {}, just exit here", error);
		}
	};

	println!("file content: {}", content);
}
