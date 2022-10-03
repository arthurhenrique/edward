use std::io::{self, LineWriter};
use std::io::Error;
use std::fs::{File, OpenOptions};
use std::path::Path;

pub fn r_buffer<P>(filename: P) -> Result<io::BufReader<File>, Error>
where
    P: AsRef<Path>,
{
    match File::open(filename)  {
        Ok(file) => Ok(io::BufReader::new(file)),
        Err(reason) => Err(reason)
    }
}

pub fn w_buffer<P>(filename: P, size: usize) -> Result<io::LineWriter<File>, Error>
where
    P: AsRef<Path>,
{
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(filename);

    match file {
        Ok(file) => Ok(LineWriter::with_capacity(size, file)),
        Err(reason) => Err(reason)
    }
}
