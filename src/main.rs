mod edward;

use edward::split;

use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// A command with positional arguments.
struct WithPositional {
    #[argh(positional)]
    path: String,
}

fn main() -> std::io::Result<()> {
    let arg = argh::from_env::<WithPositional>();
    let file_path = &arg.path;
    split(file_path);

    Ok(())
}
