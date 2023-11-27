use clap::Parser;
use std::io::BufRead;
use anyhow::{
    Context,
    Result,
};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };

    let args = Cli::parse();

    // let content = std::fs::read_to_string(&args.path)
    //     .expect("could not read file");

    let f = std::fs::File::open(&args.path)
        .with_context(|| format!("could not open file `{}`", args.path.display()))?;
    let reader = std::io::BufReader::new(f);

    for line in reader.lines() {
        if line.as_ref().expect("not a line").contains(&args.pattern) {
            println!("{}", line?);
        }
    }

    Ok(())
}
