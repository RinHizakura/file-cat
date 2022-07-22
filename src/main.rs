mod cat;
mod err;

use crate::cat::CatRunner;
use crate::err::*;
use std::fs::File;

fn open_file(path: &str) -> Result<File> {
    let file = File::open(path)?;

    Ok(file)
}

fn main() {
    let file = open_file("src/main.rs").expect("Invalid path");

    let mut cat_runner = CatRunner::new(file);

    cat_runner.run_simple().expect("run_simple fail");
    cat_runner.run_fast().expect("run fast fail");
}
