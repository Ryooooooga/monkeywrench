use std::cmp;

use super::{VersionArgs, VersionIncArgs, VersionSubcommand};

fn increment_version(version: &str, level: u32) -> anyhow::Result<String> {
    if level == 0 {
        return Err(anyhow::Error::msg("Level must be greater than 0"));
    }

    let digits_pattern = regex::Regex::new("[0-9]+").unwrap();
    let matches: Vec<_> = digits_pattern.find_iter(version).collect();
    if matches.is_empty() {
        return Err(anyhow::Error::msg("No digits found in version"));
    }

    let max_level = matches.len();
    let index = max_level - cmp::min(level as usize, max_level);

    let mut next_version = String::new();
    next_version.push_str(&version[..matches[index].start()]);

    let n = matches[index].as_str().parse::<i32>()?;
    next_version.push_str(format!("{}", n + 1).as_str());

    let mut last_index = matches[index].end();
    for m in &matches[index + 1..] {
        next_version.push_str(&version[last_index..m.start()]);
        next_version.push('0');

        last_index = m.end();
    }

    next_version.push_str(&version[last_index..]);

    Ok(next_version)
}

#[test]
fn test_increment_version() {
    assert_eq!(increment_version("v1", 1).unwrap(), "v2");
    assert_eq!(increment_version("v2", 1).unwrap(), "v3");
    assert_eq!(increment_version("v2", 2).unwrap(), "v3");

    assert_eq!(increment_version("1", 1).unwrap(), "2");
    assert_eq!(increment_version("2", 1).unwrap(), "3");
    assert_eq!(increment_version("2", 2).unwrap(), "3");

    assert_eq!(increment_version("v1.2.3", 1).unwrap(), "v1.2.4");
    assert_eq!(increment_version("v1.2.3", 2).unwrap(), "v1.3.0");
    assert_eq!(increment_version("v1.2.3", 3).unwrap(), "v2.0.0");

    assert_eq!(increment_version("1.2.3", 1).unwrap(), "1.2.4");
    assert_eq!(increment_version("1.2.3", 2).unwrap(), "1.3.0");
    assert_eq!(increment_version("1.2.3", 3).unwrap(), "2.0.0");

    assert_eq!(
        increment_version("v1.2.3-alpha.4", 1).unwrap(),
        "v1.2.3-alpha.5"
    );
    assert_eq!(
        increment_version("v1.2.3-alpha.4", 2).unwrap(),
        "v1.2.4-alpha.0"
    );
    assert_eq!(
        increment_version("v1.2.3-alpha.4", 3).unwrap(),
        "v1.3.0-alpha.0"
    );
    assert_eq!(
        increment_version("v1.2.3-alpha.4", 4).unwrap(),
        "v2.0.0-alpha.0"
    );
}

fn inc(args: &VersionIncArgs) -> anyhow::Result<()> {
    println!(
        "{}",
        increment_version(&args.version, args.level)
            .as_ref()
            .unwrap_or(&args.version)
    );
    Ok(())
}

pub fn run(args: &VersionArgs) -> anyhow::Result<()> {
    match &args.subcommand {
        VersionSubcommand::Inc(args) => inc(args),
    }
}
