extern crate ctrlc;
extern crate anyhow;

use ctrlc::set_handler;
use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver, select};
use anyhow::{Context, Ok, Result};
use clap::Parser;

/// search for  a pattern  in a input file and display the line that contains it.
#[derive(Parser)]
struct Cli {
    // pattern to look for, as a str.
    pattern: String,
    // path to read file
    path: std::path::PathBuf,
}

// signal ctrl + c
fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error>{
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;
    Ok(receiver);
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}` ", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
fn find_matches(content: &str, pattern: &str, mut writer:impl std::io::Write){
    for line in content.lines(){
        if line.contains(pattern){
            writeln!(writer, "{}", line);
        }
    }
}
#[test] ///  It allows the build system to discover such functions and run them as tests, verifying that they don’t panic.
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

// commented fn of ctrl + c

// fn main() -> Result<()> {
//     let ctrl_c_events = ctrl_channel()?;
//     let ticks = tick(Duration::from_secs(1));
//
//     loop {
//         select! {
//             recv(ticks) -> _ => {
//                 println!("working!");
//             }
//             recv(ctrl_c_events) -> _ => {
//                 println!();
//                 println!("Goodbye!");
//                 break;
//             }
//         }
//     }
//
//     Ok(())
// }
