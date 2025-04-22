use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about = "The rcp utility copies the contents of the source_file to the target_file.", long_about = None)]
struct Args {
    #[arg(
        short,
        help = "For each existing destination pathname, remove it and create a new file, without prompting for confirmation."
    )]
    f: bool,

    #[arg(
        short,
        help = "Cause rcp to write a prompt to the standard error output before copying a file that would overwrite an existing file."
    )]
    i: bool,

    #[arg(short, help = "Do not overwrite an existing file.")]
    n: bool,

    #[arg(
        short,
        help = "Cause rcp to be verbose, showing files as they are copied."
    )]
    v: bool,

    source_file: String,
    target_file: String,
}

fn handle_error(e: io::Error, context: &str, error_code: i32) -> ! {
    eprintln!("{}: {}", context, e);
    std::process::exit(error_code);
}

fn confirmation_received(dst: &str) -> bool {
    eprint!("overwrite {}? (y/n [n]) ", dst);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let response = input.trim().to_lowercase() == "y";
    if !response {
        eprintln!("not overwritten");
    }
    response
}

fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn is_same_file(src: &str, dst: &str) -> bool {
    if let (Ok(_src_meta), Ok(_dst_meta)) = (fs::metadata(src), fs::metadata(dst)) {
        if let (Ok(src_canon), Ok(dst_canon)) = (fs::canonicalize(src), fs::canonicalize(dst)) {
            return src_canon == dst_canon;
        }
    }
    false
}

fn can_copy(args: &Args) -> bool {
    // Check if source and target are the same file
    if file_exists(&args.target_file) && is_same_file(&args.source_file, &args.target_file) {
        eprintln!(
            "rcp: {} and {} are identical (not copied).",
            args.source_file, args.target_file
        );
        return false;
    }

    // If target doesn't exist, we can copy
    if !file_exists(&args.target_file) {
        return true;
    }

    // Handle flags with proper precedence
    if args.n {
        return false;
    }

    if args.i {
        return confirmation_received(&args.target_file);
    }

    // Default is to overwrite if -f is set or no flags are provided
    true
}

fn copy_file(src: &str, dst: &str, verbose: bool) -> io::Result<()> {
    let mut src_file = fs::File::open(src)?;
    let mut contents = Vec::new();
    src_file.read_to_end(&mut contents)?;

    let mut dst_file = fs::File::create(dst)?;
    dst_file.write_all(&contents)?;

    if verbose {
        println!("{} -> {}", src, dst);
    }

    Ok(())
}

fn main() {
    let args = Args::parse();

    // Check if source file exists
    if !file_exists(&args.source_file) {
        eprintln!("rcp: {}: No such file or directory", &args.source_file);
        std::process::exit(1);
    }

    // Check if source is a directory
    if Path::new(&args.source_file).is_dir() {
        eprintln!("rcp: {}: Is a directory", &args.source_file);
        std::process::exit(1);
    }

    if !can_copy(&args) {
        std::process::exit(0);
    }

    if let Err(e) = copy_file(&args.source_file, &args.target_file, args.v) {
        handle_error(
            e,
            &format!(
                "rcp: failed to copy {} to {}",
                &args.source_file, &args.target_file
            ),
            1,
        );
    }
}
