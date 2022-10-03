use std::io::{BufRead, Error, Write};

use crate::file::{r_buffer, w_buffer};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub fn split(file_path: &str, chunk_size: &u32, prefix_name: &str) -> Result<(), Error> {
    let input = r_buffer(file_path)?;

    let lines = input.lines().flatten();

    for (line_idx, line) in lines.enumerate() {
        let sufix = line_idx as u32 / chunk_size;
        let file_name = format!("{}{:03}", prefix_name, sufix);
        let chunk_size = chunk_size.clone() as usize;
        let mut w_buf = w_buffer(&file_name, chunk_size)?;
        w_buf.write(&format!("{}{}", line, LINE_ENDING).as_bytes())?;
    }

    return Ok(());
}

#[test]
fn test_odd() {
    use std::fs;
    use std::fs::File;

    let file_name = "input";
    let chunk_size = 1;
    let _ = File::create(&file_name);
    let _ = fs::write::<&&str, String>(
        &file_name,
        "edward scissorhands\n"
            .repeat((&chunk_size * 2 + 1).try_into().unwrap())
            .try_into()
            .unwrap(),
    );
    let prefix = "x";

    split(file_name, &chunk_size, &prefix).unwrap();

    let paths = fs::read_dir("./").unwrap();
    let mut acc = 0;
    for path in paths {
        let s = format!("{}", path.unwrap().path().display());
        println!("{}", s);
        if s.starts_with(&format!("./{}", &prefix)) {
            acc += 1;
        }
    }

    fs::remove_file("input").unwrap();
    fs::remove_file("x000").unwrap();
    fs::remove_file("x001").unwrap();
    fs::remove_file("x002").unwrap();

    assert_eq!(acc, 3)
}
