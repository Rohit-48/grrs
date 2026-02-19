use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Write};
use indicatif::ProgressBar;
use log::{info, warn};

/// search for  a pattern  in a input file and display the line that contains it.
#[derive(Parser)]
struct Cli {
    // pattern to look for, as a str.
    pattern: String,
    // path to read file
    path: std::path::PathBuf,
}


fn main() -> Result<()> {
    /// Logs for our application
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");

    ///A progress bar and cli reporting lib.
    let pb  = indecafif::ProgressBar::new(100);
    for i in 0..100{
        do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("Done!");

    let args = Cli::parse();

    let stdout = io::stdout(); //get the global stdout entity
    let mut handle  = io::BufWriter::new(stdout); // optional: wrap that handles in a buffer
    writeln!(handle, "foo: {}", 42); // add '?' error caring.

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read the file `{}` ", agrs.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
        println!("{}", line);
    }
}
    Ok(())
}
