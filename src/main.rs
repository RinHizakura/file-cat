mod cat;
mod err;

use crate::cat::CatRunner;
use crate::err::*;
use nix::fcntl::*;
use std::fs::File;
use std::io;
use std::os::unix::io::AsRawFd;

fn open_file(path: &str) -> Result<File> {
    let file = File::open(path)?;

    Ok(file)
}

fn cntl_stdout() -> Result<()> {
    let stdout = io::stdout();
    let fd = stdout.lock().as_raw_fd();

    let flags = nix::fcntl::fcntl(fd, F_GETFL)?;
    let mut flags = OFlag::from_bits_truncate(flags);
    flags.remove(OFlag::O_APPEND);
    nix::fcntl::fcntl(fd, F_SETFL(flags))?;

    Ok(())
}

fn main() {
    let file = open_file("src/main.rs").expect("Invalid path");
    cntl_stdout().expect("cntl stdout fail");

    let mut cat_runner = CatRunner::new(file);
    //cat_runner.run_simple().expect("run_simple fail");
    cat_runner.run_fast().expect("run fast fail");
}
