use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Result};
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<()> {
    let opts = Cli::from_args();

    let f = File::open(&opts.path)?;
    let f = BufReader::new(f);

    for line in f.lines().filter_map(|l| l.ok()) {
        if line.contains(&opts.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
