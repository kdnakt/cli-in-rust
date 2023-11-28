use std::io::BufRead;
use anyhow::Result;

pub fn find_matches<R>(
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
    let _ = find_matches(content, "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
