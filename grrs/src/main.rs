use std::env::args;
use std::path::PathBuf;

struct Cli {
    path: PathBuf,
    pattern: String,
}

fn main() {
    let pattern = args().nth(1).expect("no pattern given");
    let path = args().nth(2).expect("no path given");

    let cli = Cli {
        pattern: pattern,
        path: PathBuf::from(path),
    };
}
