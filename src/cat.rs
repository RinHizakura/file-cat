use crate::err::*;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::io::FromRawFd;

pub struct CatRunner {
    file: File,
}

impl CatRunner {
    pub fn new(f: File) -> Self {
        CatRunner { file: f }
    }

    pub fn run_simple(&mut self) -> Result<()> {
        let stdout = io::stdout();
        let mut stdout_lock = stdout.lock();
        let mut buf = [0; 1024 * 64];
        while let Ok(n) = self.file.read(&mut buf) {
            if n == 0 {
                break;
            }
            stdout_lock.write_all(&buf[..n])?;
        }
        Ok(())
    }

    pub fn run_fast(&self) -> Result<()> {
        let stdout = io::stdout();
        let mut stdout_lock = stdout.lock();
        let (pipe_read, pipe_write) = nix::unistd::pipe()?;
        let (pipe_read, pipe_write) =
            unsafe { (File::from_raw_fd(pipe_read), File::from_raw_fd(pipe_write)) };

        Ok(())
    }
}
