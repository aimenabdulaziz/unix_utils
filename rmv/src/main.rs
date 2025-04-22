use clap::Parser;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(version, about = "The rmv utility renames the file named by the source operand to the destination path named by the
target operand.", long_about = None)]
struct Args {
    #[arg(
        short,
        help = "If the target file already exists, it will be overwritten.
             (The -f option overrides any **previous** -i or -n options.)"
    )]
    f: bool,

    #[arg(
        short,
        help = "Cause rmv to write a prompt to standard error before moving a file that would overwrite an existing file.
             If the response from the standard input begins with the character 'y' or 'Y', the move is attempted.
             (The -i option overrides any **previous** -f or -n options.)"
    )]
    i: bool,

    #[arg(
        short,
        help = "Do not overwrite an existing file.  (The -n option overrides any **previous** -f or -i options.)"
    )]
    n: bool,

    source: String,
    target: String,
}
fn handle_error(e: io::Error, context: &str, error_code: i32) -> ! {
    eprintln!("{}: {}", context, e);
    std::process::exit(error_code);
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);
}
