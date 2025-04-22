use clap::Parser;
use std::path::Path;
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

fn confirmation_received(dst: &str) -> bool {
    eprint!("overwrite {dst}? (y/n [n]) ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let response = input.trim().to_lowercase() == "y";
    if !response {
        println!("not overwritten");
    }
    response
}

// https://doc.rust-lang.org/std/path/struct.Path.html#method.exists
fn file_exists(dst: &str) -> bool {
    Path::new(dst).exists()
}
// https://doc.rust-lang.org/std/fs/fn.rename.html
fn move_file(src: &str, dst: &str) -> io::Result<()> {
    fs::rename(src, dst)
}

fn get_target_path(target: &str, source: &str) -> String {
    let target_path = Path::new(target);
    if target_path.is_dir() {
        // If target is a directory, append the source filename to it
        let source_name = Path::new(source)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();
        format!("{}/{}", target.trim_end_matches('/'), source_name)
    } else {
        target.to_string()
    }
}

fn can_move(args: &Args) -> bool {
    let target_path = get_target_path(&args.target, &args.source);

    if !file_exists(&target_path) {
        return true;
    }

    if args.n {
        return false;
    }

    if args.i {
        return confirmation_received(&args.target);
    }

    // default behavior for mv if there is no flag is to overwrite
    true
}

fn main() {
    let args = Args::parse();

    // Check if source file exists
    if !file_exists(&args.source) {
        eprintln!("mv: {}: No such file or directory", &args.source);
        std::process::exit(1);
    }

    if !can_move(&args) {
        std::process::exit(0);
    }

    // if source file does not exist, exit with error
    let target_path = get_target_path(&args.target, &args.source);
    if let Err(e) = move_file(&args.source, &target_path) {
        handle_error(
            e,
            &format!("mv: rename {} to {}", &args.source, &target_path),
            1,
        );
    }
}
