mod optimization;
mod parser;
mod transpiler;

use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bfc")]
struct Options {
    #[structopt(parse(from_os_str))]
    /// input file
    input: PathBuf,

    #[structopt(short, long, default_value = "out.c", parse(from_os_str))]
    /// output file
    out: PathBuf,
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let f = File::open(path)?;
    let mut buf_reader = BufReader::new(f);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    let opt: Options = Options::from_args();
    let r = read_file(opt.input);
    if let Ok(contents) = r {
        let parsed = parser::parse(contents.as_str());
        match parsed {
            Ok(tree) => {
                let mut tree = tree;
                tree = optimization::optimize(tree);

                let mut transpiler = transpiler::Transpiler::new(tree, opt.out);
                transpiler.transpile();
            },
            Err(e) => {
                eprintln!("error: {}", e.message);
                return;
            }
        }
    }
}
