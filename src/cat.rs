use crate::err::*;
use nix::fcntl::SpliceFFlags;
use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;

// FIXME: access from systme define
const PAGE_SIZE: usize = 0x1000;
const SPLICE_SIZE: usize = PAGE_SIZE * 16;

fn splice(source: i32, target: i32, len: usize) -> Result<usize> {
    let n = nix::fcntl::splice(source, None, target, None, len, SpliceFFlags::empty());

    let n = n?;
    Ok(n)
}

fn splice_in(source: i32, target: i32, len: usize) -> Result<usize> {
    Ok(splice(source, target, len)?)
}

fn splice_out(source: i32, target: i32, len: usize) -> Result<()> {
    let mut left = len;

    while left != 0 {
        let n = splice(source, target, left)?;
        left -= n;
    }

    Ok(())
}

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

    pub fn run_fast(&mut self) -> Result<()> {
        let stdout = io::stdout();
        let stdout_lock = stdout.lock().as_raw_fd();
        /* Creat pipe, everything written to pipe_write can be read
         * from pipe_read */
        let (pipe_read, pipe_write) = nix::unistd::pipe()?;

        loop {
            match splice_in(self.file.as_raw_fd(), pipe_write, SPLICE_SIZE) {
                Ok(n) => {
                    if n == 0 {
                        return Ok(());
                    }

                    splice_out(pipe_read, stdout_lock, n)?;
                }
                Err(_) => {
                    /* fall back to simple cat(1) implementation
                     * if we can't do splice */
                    return Ok(self.run_simple()?);
                }
            }
        }
    }
}
