use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::{Context, Result};
use grrs::find_matches;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<()> {
    let opts = Cli::from_args();

    let f =
        File::open(&opts.path).with_context(|| format!("could not read file `{:?}`", opts.path))?;
    let f = BufReader::new(f);

    find_matches(f, &opts.pattern);

    Ok(())
}
