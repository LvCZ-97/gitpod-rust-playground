use std::path::PathBuf;
use std::fs::read_to_string;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opts = Cli::from_args();
    
    let content = read_to_string(&opts.path).expect("could not read file.");
    for line in content.lines() {
        if line.contains(&opts.pattern) {
            println!("{}", line);
        }
    }
}
