mod compiler;

use compiler::Compiler;

use std::fs::File;
use std::io::{BufReader, Read, Result as IoResult};
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

fn read_file<P: AsRef<Path>>(path: P) -> IoResult<String> {
    let f = File::open(path)?;
    let mut buf_reader = BufReader::new(f);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    let opt: Options = Options::from_args();
    let r = read_file(opt.input);
    if let Ok(content) = r {
        let mut comp = Compiler::new(content, opt.out.clone());
        comp.write_to_c_file();
        let out_path_str = opt.out.into_os_string().into_string().unwrap();
        println!("info: finished writing to {}", out_path_str);
        println!("info: run gcc {} -o out && ./out", out_path_str);
    } else if let Err(e) = r {
        eprintln!("an error occurred while reading the input file: {}", e);
    }
}
