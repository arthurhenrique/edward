mod edward;
mod file;

use edward::split;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// A command with positional arguments.
struct WithPositional {
    #[argh(positional)]
    path: String,
    #[argh(option, default = "1000", description = "the size of each chunk")]
    chunk_size: u32,
    #[argh(
        option,
        default = "String::from(\"chunk_\")",
        description = "prefix of the name of chunks"
    )]
    prefix: String,
}

fn main() -> std::io::Result<()> {
    let arg = argh::from_env::<WithPositional>();
    let file_path = &arg.path;
    let chunk_size = &arg.chunk_size;
    let prefix = &arg.prefix;

    split(file_path, chunk_size, prefix)?;

    Ok(())
}
