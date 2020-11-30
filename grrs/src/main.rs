use std::env::args;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opts = Cli::from_args();

    println!("{}", opts.pattern);
    println!("{:?}", opts.path);
}
