use std::fs::read_to_string;
use std::fs::File;
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
    let mut reader = BufReader::new(f);
    

    Ok(())
}
