use std::path::PathBuf;

use eyre::{bail, Result};
use testsuite::TestSuite;

mod testsuite;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        bail!("usage: cargo test-wast <wast-file>")
    }

    // cwd for relative paths, absolute paths are kept as-is
    let cwd = std::env::current_dir()?;

    // if current dir is crates/tinywasm, then we want to go up 2 levels
    let mut wast_file = if cwd.ends_with("crates/tinywasm") {
        PathBuf::from("../../")
    } else {
        PathBuf::from("./")
    };

    wast_file.push(&args[1]);
    let wast_file = cwd.join(wast_file);

    test_wast(wast_file.to_str().expect("wast_file is not a valid path"))?;
    Ok(())
}

fn test_wast(wast_file: &str) -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    println!("args: {:?}", args);

    let mut test_suite = TestSuite::new();
    println!("running wast file: {}", wast_file);
    test_suite.run_paths(&[wast_file])?;

    if test_suite.failed() {
        eprintln!("\n\nfailed one or more tests:\n{:#?}", test_suite);
        bail!("failed one or more tests")
    } else {
        println!("\n\npassed all tests:\n{:#?}", test_suite);
        Ok(())
    }
}