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

    find_matches(reader, &args.pattern, &mut std::io::stdout())
}

fn find_matches<R>(
    content: std::io::BufReader<R>,
    pattern: &str,
    mut writer: impl std::io::Write
) -> Result<()>
where R: std::io::Read
{
    for line in content.lines() {
        if line.as_ref().expect("not a line").contains(pattern) {
            writeln!(writer, "{}", line?)?;
        }
    }
    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let content = std::io::BufReader::new("lorem ipsum\ndolor sit amet".as_bytes());
    find_matches(content, "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
