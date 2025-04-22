use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, stdin};

#[derive(Parser, Debug)]
#[command(version, about = "Display a line of text", long_about = None)]
struct Args {
    #[arg(
        short,
        default_value_t = 10,
        help = "Print count lines of each of the specified files."
    )]
    n: usize,

    // List of files
    files: Option<Vec<String>>,
}

fn handle_error(e: io::Error, context: &str, error_code: i32) -> ! {
    eprintln!("{}: {}", context, e);
    std::process::exit(error_code);
}

// Only read the first n lines (good for large files cause the whole file is not loaded into memory)
fn process_file(file_path: &str, num_lines: usize) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    println!("==> {file_path} <==");

    // Read and print the first n lines
    for (i, line) in reader.lines().enumerate() {
        if i >= num_lines {
            break;
        }
        println!("{}", line?);
    }

    Ok(())
}

fn process_stdin(num_lines: usize) -> io::Result<()> {
    // Start at 1 for each stdin session
    let mut input: String = String::new();
    let mut loaded: usize = 0;
    loop {
        match stdin().read_line(&mut input) {
            Ok(0) => break, // EOF reached
            Ok(_) => {
                print!("{}", input);
                input.clear();
                loaded += 1;

                // Break before reading more lines if we have reached the limit
                if loaded >= num_lines {
                    break;
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    let num_lines: usize = args.n;

    match args.files {
        Some(files) => {
            for (i, file) in files.iter().enumerate() {
                if let Err(e) = process_file(&file, num_lines) {
                    handle_error(e, "Error reading file", 1);
                }

                // Print a newline after each file, except for the last one (to match the behavior of `head`)
                if i < files.len() - 1 {
                    println!();
                }
            }
        }
        _ => {
            // Read and print content from stdin if no files are given
            if let Err(e) = process_stdin(num_lines) {
                handle_error(e, "Error reading from stdin", 2);
            }
        }
    }
}
