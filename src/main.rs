use clap::Parser;


#[derive(Parser)]
struct Cli {
    // pattern to look for, as a str.
    pattern: String,
    // path to read file
    path: std::path::PathBuf,
}

fn main() {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {}
        println!("{}", line)
    }
}
