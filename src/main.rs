use anyhow::{Context, Result};
use clap::Parser;

/// search for  a pattern  in a input file and display the line that contains it.
#[derive(Parser)]
struct Cli {
    // pattern to look for, as a str.
    pattern: String,
    // path to read file
    path: std::path::PathBuf,
}


fn main() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read the file `{}` ", agrs.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
        println!("{}", line);
    }
}
    Ok(())
}
